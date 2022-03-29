use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::Duration;

use basedrop::Shared;
use itertools::Itertools;

use atomic_queue::Queue;
use audio_garbage_collector::make_shared;
use audio_processor_traits::MidiMessageLike;
use augmented_atomics::{AtomicF32, AtomicOption};
use augmented_midi::{parse_midi_event, MIDIMessage, ParserState};

use crate::midi_map::{MidiControllerNumber, MidiMap};
use crate::parameters::{EntityId, ParameterValue};
use crate::MultiTrackLooper;

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MidiStoreValue {
    pub channel: u8,
    pub controller_number: u8,
    pub value: u8,
}

pub struct MidiStoreHandle {
    cc_store: Vec<Vec<AtomicOption<AtomicU8>>>,
    events: Shared<Queue<MidiStoreValue>>,
    midi_map: MidiMap,
}

impl Default for MidiStoreHandle {
    fn default() -> Self {
        Self::new(make_shared(Queue::new(100)))
    }
}

impl MidiStoreHandle {
    pub fn new(events: Shared<Queue<MidiStoreValue>>) -> Self {
        let cc_store = [[0u16; 256]; 256]
            .iter()
            .map(|_| {
                [0u16; 256]
                    .iter()
                    .map(|_| AtomicOption::empty())
                    .collect_vec()
            })
            .collect_vec();

        Self {
            cc_store,
            events,
            midi_map: MidiMap::default(),
        }
    }

    pub fn midi_map(&self) -> &MidiMap {
        &self.midi_map
    }

    pub fn queue(&self) -> &Shared<Queue<MidiStoreValue>> {
        &self.events
    }

    pub fn values(&self) -> impl Iterator<Item = MidiStoreValue> + '_ {
        self.cc_store
            .iter()
            .enumerate()
            .flat_map(|(channel, channel_values)| {
                channel_values
                    .iter()
                    .enumerate()
                    .filter_map(|(controller_number, value)| {
                        value.inner().map(|v| (controller_number as u8, v))
                    })
                    .map(move |(controller_number, value)| MidiStoreValue {
                        channel: channel as u8,
                        controller_number,
                        value,
                    })
            })
    }

    pub fn process_midi_events<Message: MidiMessageLike>(
        &self,
        midi_messages: &[Message],
        multi_track_looper: &MultiTrackLooper,
    ) {
        for message in midi_messages {
            self.push_event_to_queues(message);
            self.update_multi_track_looper(message, multi_track_looper);
        }
    }

    fn update_multi_track_looper<Message: MidiMessageLike>(
        &self,
        message: &Message,
        looper: &MultiTrackLooper,
    ) -> Option<()> {
        let bytes = message.bytes()?;
        let (_, message) = parse_midi_event::<&[u8]>(bytes, &mut ParserState::default()).ok()?;
        if let MIDIMessage::ControlChange {
            controller_number,
            value,
            ..
        } = message
        {
            let entity_id = self
                .midi_map
                .get(&MidiControllerNumber::new(controller_number))?;
            let EntityId::EntityIdLooperParameter(looper_id, parameter_id) = entity_id;
            looper.handle.set_parameter(
                looper_id,
                parameter_id,
                ParameterValue::Float(AtomicF32::new(value as f32 / 127.0)),
            );
        }

        Some(())
    }

    fn push_event_to_queues<Message: MidiMessageLike>(&self, midi_message: &Message) {
        let event = midi_message
            .bytes()
            .map(|bytes| parse_midi_event::<&[u8]>(bytes, &mut ParserState::default()).ok())
            .flatten()
            .map(|(_, event)| event);

        if let Some(event) = event {
            if let MIDIMessage::ControlChange {
                channel,
                controller_number,
                value,
            } = event
            {
                self.cc_store[channel as usize][controller_number as usize].set(Some(value));
                self.events.push(MidiStoreValue {
                    channel,
                    controller_number,
                    value,
                });
            }
        }
    }
}

#[repr(C)]
pub enum MidiEvent {
    Value(MidiStoreValue),
}

pub struct MidiStoreActor {
    events_queue: Shared<Queue<MidiStoreValue>>,
    latest_events: VecDeque<MidiStoreValue>,
    current_cc_values: HashMap<u8, u8>,
    is_running: Shared<AtomicBool>,
    callback: Box<dyn Fn(MidiEvent) + Send>,
}

impl MidiStoreActor {
    pub fn new(
        events_queue: Shared<Queue<MidiStoreValue>>,
        is_running: Shared<AtomicBool>,
        callback: Box<dyn Fn(MidiEvent) + Send>,
    ) -> Self {
        Self {
            events_queue,
            latest_events: VecDeque::new(),
            current_cc_values: HashMap::new(),
            is_running,
            callback,
        }
    }

    pub fn run(&mut self) {
        while self.is_running.load(Ordering::Relaxed) {
            if let Some(event) = self.events_queue.pop() {
                self.on_receive_event(event);
            }

            std::thread::sleep(Duration::from_millis(50))
        }
    }

    fn on_receive_event(&mut self, event: MidiStoreValue) {
        self.current_cc_values
            .insert(event.controller_number, event.value);
        self.latest_events.push_front(event.clone());
        self.latest_events.truncate(100);
        (self.callback)(MidiEvent::Value(event.clone()));
    }
}

pub trait MidiStoreActorDelegate {
    fn on_event(&self, event: MidiStoreValue);
}

#[cfg(test)]
mod test {
    use assert_no_alloc::assert_no_alloc;
    use basedrop::Owned;
    use itertools::Itertools;

    use audio_garbage_collector::{handle, make_shared};
    use audio_processor_standalone_midi::host::{MidiMessageEntry, MidiMessageWrapper};

    use super::*;

    #[test]
    fn test_create_store() {
        let _store = MidiStoreHandle::new(make_shared(Queue::new(100)));
    }

    #[test]
    fn test_process_event() {
        let queue = make_shared(Queue::new(100));
        let store = MidiStoreHandle::new(queue.clone());
        let message = MidiMessageEntry(Owned::new(
            handle(),
            MidiMessageWrapper {
                message_data: [0b1011_0000, 55, 12],
                timestamp: 0,
            },
        ));

        assert_no_alloc(|| {
            store.push_event_to_queues(&message);
        });

        let values = store.values().collect_vec();
        assert_eq!(values.len(), 1);
        assert_eq!(
            values[0],
            MidiStoreValue {
                channel: 0,
                controller_number: 55,
                value: 12
            }
        );

        let event = queue.pop().unwrap();
        assert_eq!(
            event,
            MidiStoreValue {
                channel: 0,
                controller_number: 55,
                value: 12,
            }
        );
    }

    #[test]
    fn test_process_events() {
        let queue = make_shared(Queue::new(100));
        let store = MidiStoreHandle::new(queue.clone());
        let make_message = || {
            MidiMessageEntry(Owned::new(
                handle(),
                MidiMessageWrapper {
                    message_data: [0b1011_0000, 55, 12],
                    timestamp: 0,
                },
            ))
        };

        let looper = MultiTrackLooper::default();
        let events = [make_message(), make_message(), make_message()];
        assert_no_alloc(|| {
            store.process_midi_events(&events, &looper);
        });

        let values = store.values().collect_vec();
        assert_eq!(values.len(), 1);
        assert_eq!(
            values[0],
            MidiStoreValue {
                channel: 0,
                controller_number: 55,
                value: 12,
            }
        );

        for _i in 0..3 {
            let event = queue.pop().unwrap();
            assert_eq!(
                event,
                MidiStoreValue {
                    channel: 0,
                    controller_number: 55,
                    value: 12,
                }
            );
        }
    }

    #[test]
    fn test_actor_state() {
        let queue = make_shared(Queue::new(100));
        let store = MidiStoreHandle::new(queue.clone());

        let actor_is_running = make_shared(AtomicBool::new(true));
        let mut actor = MidiStoreActor::new(queue, actor_is_running.clone(), Box::new(|_| {}));

        let handle = std::thread::spawn(move || actor.run());
        let message = MidiMessageEntry(Owned::new(
            audio_garbage_collector::handle(),
            MidiMessageWrapper {
                message_data: [0b1011_0000, 55, 12],
                timestamp: 0,
            },
        ));
        assert_no_alloc(|| {
            store.push_event_to_queues(&message);
        });
        assert_no_alloc(|| {
            store.push_event_to_queues(&message);
        });
        assert_no_alloc(|| {
            store.push_event_to_queues(&message);
        });

        actor_is_running.store(false, Ordering::Relaxed);
        handle.join().unwrap();
    }

    mod midi_integration_test {
        use std::time::Duration;

        // This tests E2E:
        // * Starting the MIDI host
        // * Starting a MIDI output connection with the macOS IAC driver
        // * Sending the IAC driver a message
        // * Expecting the MIDI host received the message
        // * Pulling the message out of the audio-thread queue
        // * Parsing it and verifying it's the same
        #[actix::test]
        async fn test_receiving_events_will_cause() {
            use actix::Actor;
            use audio_processor_standalone_midi::audio_thread::MidiAudioThreadHandler;
            use audio_processor_standalone_midi::host;
            use audio_processor_standalone_midi::host::MidiHost;
            use augmented_midi::{parse_midi_event, MIDIMessage};

            let _ = wisual_logger::try_init_from_env();

            log::info!("Running integration test with IAC Driver");
            let midi_host = MidiHost::default();
            let midi_host = midi_host.start();
            midi_host
                .send(host::StartMessage)
                .await
                .unwrap()
                .expect("Failed to start MIDI host");
            let host::GetQueueMessageResult(queue) =
                midi_host.send(host::GetQueueMessage).await.unwrap();

            let output = midir::MidiOutput::new("looper-tests").unwrap();
            let ports = output.ports();
            let output_port = ports
                .iter()
                .find(|port| {
                    output
                        .port_name(port)
                        .unwrap()
                        .contains("audio_processor_standalone_midi")
                })
                .expect("Couldn't find virtual port");
            let mut output_connection = output
                .connect(output_port, "audio_processor_standalone_midi")
                .expect("Couldn't connect to virtual MIDI port");

            output_connection
                .send(&[0b1011_0001, 55, 80])
                .expect("Failed to send message to virtual port");
            std::thread::sleep(Duration::from_secs_f32(0.5));

            let mut midi_handler = MidiAudioThreadHandler::default();
            midi_handler.collect_midi_messages(&queue);
            let messages = midi_handler.buffer();
            assert_eq!(messages.len(), 1);
            let result =
                parse_midi_event::<&[u8]>(&messages[0].message_data, &mut Default::default())
                    .expect("Failed to parse event")
                    .1;
            assert!(matches!(result, MIDIMessage::ControlChange { .. }));
            if let MIDIMessage::ControlChange {
                channel,
                value,
                controller_number,
            } = result
            {
                assert_eq!(channel, 1);
                assert_eq!(value, 80);
                assert_eq!(controller_number, 55);
            }
        }
    }
}
