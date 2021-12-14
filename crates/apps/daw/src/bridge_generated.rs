#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use crate::api::*;
use flutter_rust_bridge::*;

// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_initialize_logger(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "initialize_logger",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| initialize_logger(),
    )
}

#[no_mangle]
pub extern "C" fn wire_initialize_audio(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "initialize_audio",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| initialize_audio(),
    )
}

#[no_mangle]
pub extern "C" fn wire_start_playback(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "start_playback",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| start_playback(),
    )
}

#[no_mangle]
pub extern "C" fn wire_stop_playback(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "stop_playback",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| stop_playback(),
    )
}

#[no_mangle]
pub extern "C" fn wire_set_vst_file_path(port: i64, path: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_vst_file_path",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            move |task_callback| set_vst_file_path(api_path)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_set_input_file_path(port: i64, path: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_input_file_path",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            move |task_callback| set_input_file_path(api_path)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_audio_io_get_input_devices(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_io_get_input_devices",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| audio_io_get_input_devices(),
    )
}

#[no_mangle]
pub extern "C" fn wire_get_events_sink(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_events_sink",
            port: Some(port),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| get_events_sink(task_callback.stream_sink()),
    )
}

#[no_mangle]
pub extern "C" fn wire_audio_thread_set_options(
    port: i64,
    output_device_id: *mut wire_uint_8_list,
    input_device_id: *mut wire_uint_8_list,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_thread_set_options",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_output_device_id = output_device_id.wire2api();
            let api_input_device_id = input_device_id.wire2api();
            move |task_callback| audio_thread_set_options(api_output_device_id, api_input_device_id)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_audio_graph_setup(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_graph_setup",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| audio_graph_setup(),
    )
}

#[no_mangle]
pub extern "C" fn wire_audio_graph_get_system_indexes(port: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_graph_get_system_indexes",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| audio_graph_get_system_indexes(),
    )
}

#[no_mangle]
pub extern "C" fn wire_audio_graph_connect(port: i64, input_index: u32, output_index: u32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_graph_connect",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input_index = input_index.wire2api();
            let api_output_index = output_index.wire2api();
            move |task_callback| audio_graph_connect(api_input_index, api_output_index)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_audio_node_create(port: i64, audio_processor_name: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_node_create",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_audio_processor_name = audio_processor_name.wire2api();
            move |task_callback| audio_node_create(api_audio_processor_name)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_audio_node_set_parameter(
    port: i64,
    audio_node_id: i32,
    parameter_name: *mut wire_uint_8_list,
    parameter_value: f32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_node_set_parameter",
            port: Some(port),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_audio_node_id = audio_node_id.wire2api();
            let api_parameter_name = parameter_name.wire2api();
            let api_parameter_value = parameter_value.wire2api();
            move |task_callback| {
                audio_node_set_parameter(api_audio_node_id, api_parameter_name, api_parameter_value)
            }
        },
    )
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        if self.is_null() {
            None
        } else {
            Some(self.wire2api())
        }
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: impl IntoDart

// Section: executor
support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
