// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

/*!
* Minimal binding for OpenAL.
* Bind only functions which are needed by lib sailor
*/

#![allow(dead_code, non_snake_case)]

#[link(name = "openal")]
extern "C" {}

pub mod ffi {

    use libc::{c_char, c_void, intptr_t};

    /// OpenAL types
    pub type ALboolean = c_char;
    pub type ALCboolean = c_char;
    pub type ALCdevicePtr = intptr_t;
    pub type ALCcontextPtr = intptr_t;
    pub const AL_TRUE: ALboolean = 1;
    pub const AL_FALSE: ALboolean = 0;
    pub const ALC_TRUE: ALCboolean = 1;
    pub const ALC_FALSE: ALCboolean = 0;

    /// Sound modifier
    pub const AL_GAIN: i32 = 0x100A;
    pub const AL_PITCH: i32 = 0x1003;
    pub const AL_SOURCE_RELATIVE: i32 = 0x202;
    pub const AL_POSITION: i32 = 0x1004;
    pub const AL_ORIENTATION: i32 = 0x100F;
    pub const AL_DIRECTION: i32 = 0x1005;
    pub const AL_VELOCITY: i32 = 0x1006;
    pub const AL_LOOPING: i32 = 0x1007;
    pub const AL_MIN_GAIN: i32 = 0x100D;
    pub const AL_MAX_GAIN: i32 = 0x100E;
    pub const AL_MAX_DISTANCE: i32 = 0x1023;
    pub const AL_REFERENCE_DISTANCE: i32 = 0x1020;
    pub const AL_ROLLOFF_FACTOR: i32 = 0x1021;
    pub const AL_SEC_OFFSET: i32 = 0x1024;
    pub const AL_SAMPLE_OFFSET: i32 = 0x1025;
    pub const AL_BYTE_OFFSET: i32 = 0x1026;

    /// Sound format
    pub const AL_FORMAT_MONO16: i32 = 0x1101;
    pub const AL_FORMAT_STEREO16: i32 = 0x1103;
    pub const AL_FORMAT_51CHN16: i32 = 0x120B;
    pub const AL_FORMAT_61CHN16: i32 = 0x120E;
    pub const AL_FORMAT_71CHN16: i32 = 0x1211;
    pub const AL_FORMAT_QUAD16: i32 = 0x1205;

    /// Source params
    pub const AL_BUFFER: i32 = 0x1009;
    pub const AL_BUFFERS_PROCESSED: i32 = 0x1016;
    pub const AL_BUFFERS_QUEUED: i32 = 0x1015;
    pub const AL_DIRECT_CHANNELS_SOFT: i32 = 0x1033;

    /// Source object extensions
    pub const AL_DIRECT_FILTER: i32 = 0x20005;
    pub const AL_AUXILIARY_SEND_FILTER: i32 = 0x20006;
    pub const AL_AIR_ABSORPTION_FACTOR: i32 = 0x20007;

    /// Effects
    pub const AL_EFFECT_NULL: i32 = 0x0000;
    pub const AL_EFFECT_TYPE: i32 = 0x8001;
    pub const AL_EFFECT_REVERB: i32 = 0x0001;
    pub const AL_EFFECTSLOT_NULL: i32 = 0x0000;
    pub const AL_EFFECTSLOT_EFFECT: i32 = 0x0001;
    pub const AL_EFFECTSLOT_AUXILIARY_SEND_AUTO: i32 = 0x0003;
    pub const AL_REVERB_DENSITY: i32 = 0x0001;
    pub const AL_REVERB_DIFFUSION: i32 = 0x0002;
    pub const AL_REVERB_GAIN: i32 = 0x0003;
    pub const AL_REVERB_GAINHF: i32 = 0x0004;
    pub const AL_REVERB_DECAY_TIME: i32 = 0x0005;
    pub const AL_REVERB_DECAY_HFRATIO: i32 = 0x0006;
    pub const AL_REVERB_REFLECTIONS_GAIN: i32 = 0x0007;
    pub const AL_REVERB_REFLECTIONS_DELAY: i32 = 0x0008;
    pub const AL_REVERB_LATE_REVERB_GAIN: i32 = 0x0009;
    pub const AL_REVERB_LATE_REVERB_DELAY: i32 = 0x000A;
    pub const AL_REVERB_AIR_ABSORPTION_GAINHF: i32 = 0x000B;
    pub const AL_REVERB_ROOM_ROLLOFF_FACTOR: i32 = 0x000C;
    pub const AL_REVERB_DECAY_HFLIMIT: i32 = 0x000D;

    // Filters
    pub const AL_FILTER_NULL: i32 = 0x0000;

    /// Error identifiers
    pub const AL_NO_ERROR: i32 = 0;
    pub const AL_INVALID_NAME: i32 = 0xA001;
    pub const AL_INVALID_ENUM: i32 = 0xA002;
    pub const AL_INVALID_VALUE: i32 = 0xA003;
    pub const AL_INVALID_OPERATION: i32 = 0xA004;
    pub const AL_OUT_OF_MEMORY: i32 = 0xA005;

    /// Source states
    pub const AL_SOURCE_STATE: i32 = 0x1010;
    pub const AL_INITIAL: i32 = 0x1011;
    pub const AL_PLAYING: i32 = 0x1012;
    pub const AL_PAUSED: i32 = 0x1013;
    pub const AL_STOPPED: i32 = 0x1014;

    /// ALC
    pub const ALC_CAPTURE_SAMPLES: i32 = 0x312;

    extern "C" {
        /// Context functions
        pub fn alcCreateContext(device: ALCdevicePtr, attrlist: *mut i32) -> ALCcontextPtr;
        pub fn alcMakeContextCurrent(context: ALCcontextPtr) -> ALCboolean;
        pub fn alcDestroyContext(context: ALCcontextPtr);
        pub fn alcGetCurrentContext() -> ALCcontextPtr;

        /// Device functions
        pub fn alcOpenDevice(devicename: *mut c_char) -> ALCdevicePtr;
        pub fn alcCloseDevice(device: ALCdevicePtr) -> ALCboolean;

        /// Listener functions
        pub fn alListenerf(param: i32, value: f32);
        pub fn alListener3f(param: i32, value1: f32, value2: f32, value3: f32);
        pub fn alGetListenerf(param: i32, value: *mut f32);
        pub fn alGetListener3f(
            param: f32,
            value1: *mut f32,
            value2: *mut f32,
            value3: *mut f32,
        );
        pub fn alListenerfv(param: i32, values: *const f32);
        pub fn alGetListenerfv(param: i32, values: *mut f32);

        /// Sources functions
        pub fn alGenSources(n: i32, sources: *mut u32);
        pub fn alDeleteSources(n: i32, buffers: *mut u32);
        pub fn alSourcei(source: u32, param: i32, value: i32);
        pub fn alSource3i(source: u32, param: i32, value1: i32, value2: i32, value3: i32);
        pub fn alSourcef(source: u32, param: i32, value: f32);
        pub fn alSourcePlay(source: u32);
        pub fn alSourcePause(source: u32);
        pub fn alSourceStop(source: u32);
        pub fn alGetSourcei(source: u32, param: i32, value: *mut i32);
        pub fn alGetSourcef(source: u32, param: i32, value: *mut f32);
        pub fn alSourcefv(source: u32, param: i32, value: *const f32);
        pub fn alGetSourcefv(source: u32, param: i32, value: *mut f32);
        pub fn alSourceQueueBuffers(source: u32, nb: i32, buffers: *const u32);
        pub fn alSourceUnqueueBuffers(source: u32, nb: i32, buffers: *mut u32);

        /// Sound capture functions
        pub fn alcCaptureCloseDevice(device: ALCdevicePtr) -> ALCboolean;
        pub fn alcCaptureOpenDevice(
            device: *mut c_char,
            sample_rate: i32,
            format: i32,
            buffer_size: i32,
        ) -> ALCdevicePtr;
        pub fn alcCaptureStart(devide: ALCdevicePtr);
        pub fn alcCaptureStop(devide: ALCdevicePtr);
        pub fn alcGetIntegerv(devide: ALCdevicePtr, param: i32, size: i32, values: *mut i32);
        pub fn alcCaptureSamples(devide: ALCdevicePtr, buffer: *mut c_void, sample: i32);

        /// Effects functions
        pub fn alGenAuxiliaryEffectSlots(n: i32, effect_slots: *mut u32);
        pub fn alGenEffects(n: i32, effects: *mut u32);
        pub fn alDeleteAuxiliaryEffectSlots(n: i32, effect_slots: *mut u32);
        pub fn alDeleteEffects(n: i32, effects: *mut u32);
        pub fn alIsAuxiliaryEffectSlot(source: u32) -> ALboolean;
        pub fn alAuxiliaryEffectSloti(source: u32, param: i32, value: u32);
        pub fn alEffecti(source: u32, param: i32, value: i32);
        pub fn alEffectf(source: u32, param: i32, value: f32);

        /// extension check
        pub fn alIsExtensionPresent(extension: *const c_char) -> ALboolean;
        pub fn alcIsExtensionPresent(device: ALCdevicePtr, extension: *const c_char) -> ALCboolean;

        /// Buffers functions
        pub fn alGenBuffers(n: i32, buffers: *mut u32);
        pub fn alDeleteBuffers(n: i32, buffers: *mut u32);
        pub fn alBufferData(
            buffer: u32,
            format: i32,
            data: *mut c_void,
            size: i32,
            freq: i32,
        );

        /// Error
        pub fn alGetError() -> i32;
    }

    #[repr(C)]
    pub struct ALCdevice;
    #[repr(C)]
    pub struct ALCcontext;
}

pub mod al {

    use super::ffi;
    use libc::c_void;
    use std::error::Error;
    use std::fmt;

    pub fn alBufferData(buffer: u32, format: i32, data: *mut c_void, size: i32, freq: i32) {
        unsafe {
            ffi::alBufferData(buffer, format, data, size, freq);
        }
    }

    pub fn alSourceQueueBuffers(source: u32, nb: i32, buffers: *const u32) {
        unsafe {
            ffi::alSourceQueueBuffers(source, nb, buffers);
        }
    }

    pub fn alSourcePlay(source: u32) {
        unsafe {
            ffi::alSourcePlay(source);
        }
    }

    pub fn alGetSourcei(source: u32, param: i32, value: *mut i32) {
        unsafe {
            ffi::alGetSourcei(source, param, value);
        }
    }

    pub fn alGetSourcef(source: u32, param: i32, value: *mut f32) {
        unsafe {
            ffi::alGetSourcef(source, param, value);
        }
    }

    pub fn alGetState(source: u32) -> i32 {
        let mut i = 0;
        unsafe {
            ffi::alGetSourcei(source, ffi::AL_SOURCE_STATE, &mut i);
        }
        i
    }

    pub fn alSourcei(source: u32, param: i32, value: i32) {
        unsafe {
            ffi::alSourcei(source, param, value);
        }
    }

    pub fn alSource3i(source: u32, param: i32, value1: i32, value2: i32, value3: i32) {
        unsafe {
            ffi::alSource3i(source, param, value1, value2, value3);
        }
    }

    pub fn alSourcef(source: u32, param: i32, value: f32) {
        unsafe {
            ffi::alSourcef(source, param, value);
        }
    }

    pub fn alSourcePause(source: u32) {
        unsafe {
            ffi::alSourcePause(source);
        }
    }

    pub fn alSourceStop(source: u32) {
        unsafe {
            ffi::alSourceStop(source);
        }
    }

    pub fn alSourceUnqueueBuffers(source: u32, nb: i32, buffers: *mut u32) {
        unsafe {
            ffi::alSourceUnqueueBuffers(source, nb, buffers);
        }
    }

    pub fn alGenSources(n: i32, sources: *mut u32) {
        unsafe {
            ffi::alGenSources(n, sources);
        }
    }

    pub fn alSourcefv(source: u32, param: i32, value: *const f32) {
        unsafe {
            ffi::alSourcefv(source, param, value);
        }
    }

    pub fn alGetSourcefv(source: u32, param: i32, value: *mut f32) {
        unsafe {
            ffi::alGetSourcefv(source, param, value);
        }
    }

    pub fn alGenBuffers(n: i32, buffers: *mut u32) {
        unsafe {
            ffi::alGenBuffers(n, buffers);
        }
    }

    pub fn alListenerf(param: i32, value: f32) {
        unsafe {
            ffi::alListenerf(param, value);
        }
    }

    pub fn alListener3f(param: i32, value1: f32, value2: f32, value3: f32) {
        unsafe {
            ffi::alListener3f(param, value1, value2, value3);
        }
    }

    pub fn alGetListenerf(param: i32, value: *mut f32) {
        unsafe {
            ffi::alGetListenerf(param, value);
        }
    }

    pub fn alGetListener3f(param: f32, value1: *mut f32, value2: *mut f32, value3: *mut f32) {
        unsafe {
            ffi::alGetListener3f(param, value1, value2, value3);
        }
    }

    pub fn alListenerfv(param: i32, values: *const f32) {
        unsafe {
            ffi::alListenerfv(param, values);
        }
    }

    pub fn alGetListenerfv(param: i32, values: *mut f32) {
        unsafe {
            ffi::alGetListenerfv(param, values);
        }
    }

    /// Effects functions
    pub fn alGenAuxiliaryEffectSlots(n: i32, effect_slots: *mut u32) {
        unsafe {
            ffi::alGenAuxiliaryEffectSlots(n, effect_slots);
        }
    }

    pub fn alGenEffects(n: i32, effects: *mut u32) {
        unsafe {
            ffi::alGenEffects(n, effects);
        }
    }

    pub fn alAuxiliaryEffectSloti(source: u32, param: i32, value: u32) {
        unsafe {
            ffi::alAuxiliaryEffectSloti(source, param, value);
        }
    }

    pub fn alEffecti(source: u32, param: i32, value: i32) {
        unsafe {
            ffi::alEffecti(source, param, value);
        }
    }

    pub fn alEffectf(source: u32, param: i32, value: f32) {
        unsafe {
            ffi::alEffectf(source, param, value);
        }
    }

    /// Any error that can happen during an OpenAL call.
    pub struct AlError(i32);

    impl AlError {
        /// Create a new AlError from one of the ffi::AL_* enum values.
        pub fn new(err: i32) -> AlError {
            AlError(err)
        }
    }

    impl fmt::Display for AlError {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(
                fmt,
                "{}",
                match self.0 {
                    ffi::AL_INVALID_NAME => "invalid name paramater passed to AL call",
                    ffi::AL_INVALID_ENUM => "invalid enum parameter passed to AL call",
                    ffi::AL_INVALID_VALUE => "invalid value parameter passed to AL call",
                    ffi::AL_INVALID_OPERATION => "illegal AL call",
                    ffi::AL_OUT_OF_MEMORY => "not enough memory",
                    _ => "unknow error",
                }
            )
        }
    }

    impl fmt::Debug for AlError {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(self, fmt)
        }
    }

    impl Error for AlError {}

    pub fn openal_has_error() -> Option<AlError> {
        match unsafe { ffi::alGetError() } {
            ffi::AL_NO_ERROR => None,
            err => Some(AlError::new(err)),
        }
    }

    pub fn get_channels_format(channels: i32) -> Option<i32> {
        match channels {
            1 => Some(ffi::AL_FORMAT_MONO16),
            2 => Some(ffi::AL_FORMAT_STEREO16),
            4 => Some(ffi::AL_FORMAT_QUAD16),
            5 => Some(ffi::AL_FORMAT_51CHN16),
            6 => Some(ffi::AL_FORMAT_61CHN16),
            7 => Some(ffi::AL_FORMAT_71CHN16),
            _ => None,
        }
    }
}
