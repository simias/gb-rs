/// This file contains the libretro definitions ported from `libretro.h`
///
/// For more details see the original well-commented C header file:
/// https://github.com/libretro/RetroArch/blob/master/libretro.h
///
/// I took the liberty to "rustify" the calling convention: I dropped
/// the `retro_` prefix (useless when you have namespaces) and
/// CamelCased the struct names.
///
/// Callback typedefs are altered in the same way and suffixed with
/// `Fn` for clarity.

use std::ptr;
use std::ffi::{CString, CStr};
use std::path::Path;
use std::str::FromStr;

use libc::{c_void, c_char, c_uint, c_float, c_double, size_t, int16_t};

/// Global CPU instance holding our emulator state
static mut instance: *mut ::cpu::Cpu = 0 as *mut ::cpu::Cpu;

#[repr(C)]
pub struct SystemInfo {
   pub library_name: *const c_char,
   pub library_version: *const c_char,
   pub valid_extensions: *const c_char,
   pub need_fullpath: bool,
   pub block_extract: bool,
}

#[repr(C)]
pub struct GameGeometry {
    pub base_width: c_uint,
    pub base_height: c_uint,
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub aspect_ratio: c_float,
}

#[repr(C)]
pub struct SystemTiming {
    pub fps: c_double,
    pub sample_rate: c_double,
}

#[repr(C)]
pub struct SystemAvInfo {
    pub geometry: GameGeometry,
    pub timing: SystemTiming,
}


pub type EnvironmentFn =
    unsafe extern "C" fn(cmd: c_uint, data: *mut c_void);

pub type VideoRefreshFn =
    unsafe extern "C" fn(data: *const c_void,
                         width: c_uint,
                         height: c_uint,
                         pitch: size_t);
pub type AudioSampleFn =
    extern "C" fn(left: int16_t, right: int16_t);

pub type AudioSampleBatchFn =
    unsafe extern "C" fn(data: *const int16_t,
                         frames: size_t) -> size_t;

pub type InputPollFn = extern "C" fn();

pub type InputStateFn =
    extern "C" fn(port: c_uint,
                  device: c_uint,
                  index: c_uint,
                  id:c_uint) -> int16_t;

#[repr(C)]
pub struct GameInfo {
    path: *const c_char,
    data: *const c_void,
    size: size_t,
    meta: *const c_char,
}

#[repr(C)]
pub struct Variable {
    key: *const c_char,
    value: *const c_char,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    GetVariable = 15,
    SetVariables = 16,
    GetVariableUpdate = 17,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputDevice {
    None = 0,
    JoyPad = 1,
    Mouse = 2,
    Keyboard = 3,
    LightGun = 4,
    Analog = 5,
    Pointer = 6,
}

/// RETRO_DEVICE_ID_JOYPAD_* constants
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JoyPadButton {
    B = 0,
    Y = 1,
    Select = 2,
    Start = 3,
    Up = 4,
    Down = 5,
    Left = 6,
    Right = 7,
    A = 8,
    X = 9,
    L = 10,
    R = 11,
    L2 = 12,
    R2 = 13,
    L3 = 14,
    R3 = 15,
}

//*******************************************
// Libretro callbacks loaded by the frontend
//*******************************************

static mut video_refresh: VideoRefreshFn = dummy::video_refresh;
static mut input_poll: InputPollFn = dummy::input_poll;
static mut input_state: InputStateFn = dummy::input_state;
static mut audio_sample_batch: AudioSampleBatchFn = dummy::audio_sample_batch;
static mut environment: EnvironmentFn = dummy::environment;

//*******************************
// Higher level helper functions
//*******************************

pub fn frame_done(frame: [u16; 256*144]) {
    unsafe {
        let data = frame.as_ptr() as *const c_void;

        video_refresh(data, 256, 144, 256 * 2);
    }
}

pub fn send_audio_samples(samples: &[i16]) {
    if samples.len() & 1 != 0 {
        panic!("Received an odd number of audio samples!");
    }

    let frames = (samples.len() / 2) as size_t;

    let r = unsafe {
        audio_sample_batch(samples.as_ptr(), frames)
    };

    if r != frames {
        panic!("Frontend didn't use all our samples! ({} != {})", r, frames);
    }
}

pub fn button_pressed(b: JoyPadButton) -> bool {
    unsafe {
        input_state(0,
                    InputDevice::JoyPad as c_uint,
                    0,
                    b as c_uint) != 0
    }
}

//**********************************************
// Libretro entry points called by the frontend
//**********************************************

#[no_mangle]
pub extern "C" fn retro_api_version() -> c_uint {
    // We implement the version 1 of the API
    1
}

#[no_mangle]
pub extern "C" fn retro_set_environment(callback: EnvironmentFn) {
    unsafe {
        environment = callback;

        let variables = [
            Variable { key: b"gbrs-ws_shift\0".as_ptr() as *const i8,
                       value: b"Widescreen shift (pixels); 0|8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160\0"
                       .as_ptr() as *const i8 },
            Variable { key: ptr::null() as *const i8, value: ptr::null() as *const i8 },
            ];

        environment(Environment::SetVariables as c_uint,
                        variables.as_ptr() as *mut c_void);
    }
}

#[no_mangle]
pub extern "C" fn retro_set_video_refresh(callback: VideoRefreshFn) {
    unsafe {
        video_refresh = callback
    }
}

#[no_mangle]
pub extern "C" fn retro_set_audio_sample(_: AudioSampleFn) {
}

#[no_mangle]
pub extern "C" fn retro_set_audio_sample_batch(callback: AudioSampleBatchFn) {
    unsafe {
        audio_sample_batch = callback
    }
}

#[no_mangle]
pub extern "C" fn retro_set_input_poll(callback: InputPollFn) {
    unsafe {
        input_poll = callback
    }
}

#[no_mangle]
pub extern "C" fn retro_set_input_state(callback: InputStateFn) {
    unsafe {
        input_state = callback
    }
}

#[no_mangle]
pub extern "C" fn retro_init() {
}

#[no_mangle]
pub extern "C" fn retro_deinit() {
}

#[no_mangle]
pub extern "C" fn retro_get_system_info(info: *mut SystemInfo) {
    let info = ptr_as_mut_ref(info).unwrap();

    // Strings must be static and, of course, 0-terminated
    *info = SystemInfo {
        library_name: b"gb-rs\0".as_ptr() as *const i8,
	library_version: b"0.3.0\0".as_ptr() as *const i8,
	valid_extensions: b"gb\0".as_ptr() as *const i8,
	need_fullpath: false,
	block_extract: false,
    }
}

#[no_mangle]
pub extern "C" fn retro_get_system_av_info(info: *mut SystemAvInfo) {
    let info = ptr_as_mut_ref(info).unwrap();

    println!("AV INFO");

    *info = SystemAvInfo {
        // XXX Dynamic me
        geometry: GameGeometry {
            base_width: 256,
            base_height: 144,
            max_width: 256,
            max_height: 144,
            aspect_ratio: -1.0,
        },
        timing: SystemTiming {
            fps: (0x400000 as f64) / (456. * 154.),
            sample_rate: (0x400000 as f64) / 95.,
        }
    }
}

#[no_mangle]
pub extern "C" fn retro_set_controller_port_device(_port: c_uint,
                                                   _device: c_uint) {
    println!("port device: {} {}", _port, _device);
}

#[no_mangle]
pub extern "C" fn retro_reset() {
    println!("retro reset");
}

static mut ws_shift: u8 = 0;

pub fn get_ws_shift() -> u8 {
    unsafe {
        ws_shift
    }
}

pub fn get_variable<T: FromStr>(var: &str) -> T {

    let cstr = CString::new(var).unwrap();

    let mut v = Variable {
        key: cstr.as_ptr(),
        value: ptr::null(),
    };

    let value =
        unsafe {
            environment(Environment::GetVariable as c_uint,
                        (&mut v) as *mut _ as *mut c_void);

            if v.value.is_null() {
                panic!("Couldn't get variable {}", var);
            }

            CStr::from_ptr(v.value).to_str().unwrap()
        };

    FromStr::from_str(value).ok().unwrap()
}

pub fn variables_need_update() -> bool {
    let mut needs_update = false;

    unsafe {
        environment(Environment::GetVariableUpdate as c_uint,
                    (&mut needs_update) as *mut _ as *mut c_void);
    }

    needs_update
}

#[no_mangle]
pub unsafe extern "C" fn retro_run() {
    input_poll();

    // Update variables if needed
    if variables_need_update() {
        ws_shift = get_variable("gbrs-ws_shift");
    }

    ::render_frame(ptr_as_mut_ref(instance).unwrap());
}

#[no_mangle]
pub extern "C" fn retro_serialize_size() -> size_t {
    0
}

#[no_mangle]
pub extern "C" fn retro_serialize(_data: *mut c_void,
                                  _size: size_t) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn retro_unserialize(_data: *const c_void,
                                    _size: size_t) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn retro_cheat_reset() {
}

#[no_mangle]
pub fn retro_cheat_set(_index: c_uint,
                       _enabled: bool,
                       _code: *const c_char) {
}

#[no_mangle]
pub extern "C" fn retro_load_game(info: *const GameInfo) -> bool {
    let info = ptr_as_ref(info).unwrap();

    if info.path.is_null() {
        println!("No path in GameInfo!");
        return false;
    }

    let path = unsafe {
        CStr::from_ptr(info.path)
    }.to_str().unwrap();

    let cpu = Box::new(::load_game(Path::new(path)));

    unsafe {
        instance = Box::into_raw(cpu);
    }

    true
}

#[no_mangle]
pub extern "C" fn retro_load_game_special(_type: c_uint,
                                          _info: *const GameInfo,
                                          _num_info: size_t) -> bool {
    false
}

#[no_mangle]
pub unsafe extern "C" fn retro_unload_game()  {
    if !instance.is_null() {
        // Rebuild the Box to free the instance
        Box::from_raw(instance);
        instance = ptr::null_mut();
    }
}

/// Cast a mutable pointer into a mutable reference, return None if
/// it's NULL.
pub fn ptr_as_mut_ref<'a, T>(v: *mut T) -> Option<&'a mut T> {

    if v.is_null() {
        None
    } else {
        Some(unsafe { &mut *v })
    }
}

/// Cast a const pointer into a reference, return None if it's NULL.
pub fn ptr_as_ref<'a, T>(v: *const T) -> Option<&'a T> {

    if v.is_null() {
        None
    } else {
        Some(unsafe { &*v })
    }
}

#[no_mangle]
pub extern "C" fn retro_get_region() -> c_uint {
    0
}

#[no_mangle]
pub extern "C" fn retro_get_memory_data(_id: c_uint) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn retro_get_memory_size(_id: c_uint) -> size_t {
    0
}

pub mod dummy {
    //! Placeholder implementation for the libretro callback in order
    //! to catch calls to those function in the function pointer has
    //! not yet been loaded.

    use libc::{c_void, c_uint, size_t, int16_t};

    pub unsafe extern "C" fn video_refresh(_: *const c_void,
                                       _: c_uint,
                                       _: c_uint,
                                       _: size_t) {
        panic!("Called missing video_refresh callback");
    }

    pub extern "C" fn input_poll() {
        panic!("Called missing input_poll callback");
    }

    pub unsafe extern "C" fn audio_sample_batch(_: *const int16_t,
                                                _: size_t) -> size_t {
        panic!("Called missing audio_sample_batch callback");
    }

    pub extern "C" fn input_state(_: c_uint,
                                  _: c_uint,
                                  _: c_uint,
                                  _: c_uint) -> int16_t {
        panic!("Called missing input_state callback");
    }

    pub unsafe extern "C" fn environment(_: c_uint, _: *mut c_void) {
        panic!("Called missing environment callback");
    }
}
