pub use atomic_refcell::AtomicRefCell;

pub use audio::multi_track_looper::parameters;
pub use audio::multi_track_looper::parameters::EnvelopeParameter;
pub use audio::multi_track_looper::parameters::LooperId;
pub use audio::multi_track_looper::{MultiTrackLooper, MultiTrackLooperHandle};
pub use audio::processor::handle::LooperHandle as LooperProcessorHandle;
pub use audio::processor::handle::LooperHandleThread;
pub use audio::processor::handle::LooperOptions;
pub use audio::processor::handle::QuantizeMode;
pub use audio::processor::handle::QuantizeOptions;
pub use audio::processor::LooperProcessor;
pub use audio::shuffler::LoopShufflerParams;
pub use audio::shuffler::LoopShufflerProcessorHandle;
pub use audio::time_info_provider::{TimeInfo, TimeInfoProvider, TimeInfoProviderImpl};
pub use c_api::*;
pub use services::osc_server::setup_osc_server;

mod audio;
#[allow(clippy::missing_safety_doc)]
mod c_api;
mod engine;
mod services;

const MAX_LOOP_LENGTH_SECS: f32 = 10.0;
