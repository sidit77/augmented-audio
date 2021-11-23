// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments
import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'dart:ffi' as ffi;

abstract class DawUi extends FlutterRustBridgeBase<DawUiWire> {
  factory DawUi(ffi.DynamicLibrary dylib) => DawUiImpl.raw(DawUiWire(dylib));

  DawUi.raw(DawUiWire inner) : super(inner);

  Future<int> initializeLogger({dynamic hint});

  Future<int> initializeAudio({dynamic hint});

  Future<int> startPlayback({dynamic hint});

  Future<int> stopPlayback({dynamic hint});

  Future<int> setVstFilePath({required String path, dynamic hint});

  Future<int> setInputFilePath({required String path, dynamic hint});

  Future<String> audioIoGetInputDevices({dynamic hint});

  Stream<String> getEventsSink({dynamic hint});
}

// ------------------------- Implementation Details -------------------------

/// Implementations for DawUi. Prefer using DawUi if possible; but this class allows more
/// flexible customizations (such as subclassing to create an initializer, a logger, or
/// a timer).
class DawUiImpl extends DawUi {
  DawUiImpl.raw(DawUiWire inner) : super.raw(inner);

  Future<int> initializeLogger({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
          debugName: 'initialize_logger',
          callFfi: (port) => inner.wire_initialize_logger(port),
          parseSuccessData: _wire2api_i32,
          hint: hint));

  Future<int> initializeAudio({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
          debugName: 'initialize_audio',
          callFfi: (port) => inner.wire_initialize_audio(port),
          parseSuccessData: _wire2api_i32,
          hint: hint));

  Future<int> startPlayback({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
          debugName: 'start_playback',
          callFfi: (port) => inner.wire_start_playback(port),
          parseSuccessData: _wire2api_i32,
          hint: hint));

  Future<int> stopPlayback({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
          debugName: 'stop_playback',
          callFfi: (port) => inner.wire_stop_playback(port),
          parseSuccessData: _wire2api_i32,
          hint: hint));

  Future<int> setVstFilePath({required String path, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
          debugName: 'set_vst_file_path',
          callFfi: (port) =>
              inner.wire_set_vst_file_path(port, _api2wire_String(path)),
          parseSuccessData: _wire2api_i32,
          hint: hint));

  Future<int> setInputFilePath({required String path, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
          debugName: 'set_input_file_path',
          callFfi: (port) =>
              inner.wire_set_input_file_path(port, _api2wire_String(path)),
          parseSuccessData: _wire2api_i32,
          hint: hint));

  Future<String> audioIoGetInputDevices({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
          debugName: 'audio_io_get_input_devices',
          callFfi: (port) => inner.wire_audio_io_get_input_devices(port),
          parseSuccessData: _wire2api_String,
          hint: hint));

  Stream<String> getEventsSink({dynamic hint}) =>
      executeStream(FlutterRustBridgeTask(
          debugName: 'get_events_sink',
          callFfi: (port) => inner.wire_get_events_sink(port),
          parseSuccessData: _wire2api_String,
          hint: hint));

  // Section: api2wire
  ffi.Pointer<wire_uint_8_list> _api2wire_String(String raw) {
    return _api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  int _api2wire_u8(int raw) {
    return raw;
  }

  ffi.Pointer<wire_uint_8_list> _api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  // Section: api_fill_to_wire

}

// Section: wire2api
String _wire2api_String(dynamic raw) {
  return raw as String;
}

int _wire2api_i32(dynamic raw) {
  return raw as int;
}

int _wire2api_u8(dynamic raw) {
  return raw as int;
}

Uint8List _wire2api_uint_8_list(dynamic raw) {
  return raw as Uint8List;
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class DawUiWire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  DawUiWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  DawUiWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void wire_initialize_logger(
    int port,
  ) {
    return _wire_initialize_logger(
      port,
    );
  }

  late final _wire_initialize_loggerPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_initialize_logger');
  late final _wire_initialize_logger =
      _wire_initialize_loggerPtr.asFunction<void Function(int)>();

  void wire_initialize_audio(
    int port,
  ) {
    return _wire_initialize_audio(
      port,
    );
  }

  late final _wire_initialize_audioPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_initialize_audio');
  late final _wire_initialize_audio =
      _wire_initialize_audioPtr.asFunction<void Function(int)>();

  void wire_start_playback(
    int port,
  ) {
    return _wire_start_playback(
      port,
    );
  }

  late final _wire_start_playbackPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_start_playback');
  late final _wire_start_playback =
      _wire_start_playbackPtr.asFunction<void Function(int)>();

  void wire_stop_playback(
    int port,
  ) {
    return _wire_stop_playback(
      port,
    );
  }

  late final _wire_stop_playbackPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_stop_playback');
  late final _wire_stop_playback =
      _wire_stop_playbackPtr.asFunction<void Function(int)>();

  void wire_set_vst_file_path(
    int port,
    ffi.Pointer<wire_uint_8_list> path,
  ) {
    return _wire_set_vst_file_path(
      port,
      path,
    );
  }

  late final _wire_set_vst_file_pathPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_set_vst_file_path');
  late final _wire_set_vst_file_path = _wire_set_vst_file_pathPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_set_input_file_path(
    int port,
    ffi.Pointer<wire_uint_8_list> path,
  ) {
    return _wire_set_input_file_path(
      port,
      path,
    );
  }

  late final _wire_set_input_file_pathPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_set_input_file_path');
  late final _wire_set_input_file_path = _wire_set_input_file_pathPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_audio_io_get_input_devices(
    int port,
  ) {
    return _wire_audio_io_get_input_devices(
      port,
    );
  }

  late final _wire_audio_io_get_input_devicesPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_audio_io_get_input_devices');
  late final _wire_audio_io_get_input_devices =
      _wire_audio_io_get_input_devicesPtr.asFunction<void Function(int)>();

  void wire_get_events_sink(
    int port,
  ) {
    return _wire_get_events_sink(
      port,
    );
  }

  late final _wire_get_events_sinkPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_events_sink');
  late final _wire_get_events_sink =
      _wire_get_events_sinkPtr.asFunction<void Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list(
    int len,
  ) {
    return _new_uint_8_list(
      len,
    );
  }

  late final _new_uint_8_listPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list');
  late final _new_uint_8_list = _new_uint_8_listPtr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>(
          'free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct = _free_WireSyncReturnStructPtr
      .asFunction<void Function(WireSyncReturnStruct)>();

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Uint8 Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
