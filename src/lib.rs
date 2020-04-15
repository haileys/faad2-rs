use std::ffi::CStr;
use std::fmt::{self, Debug, Display};
use std::marker::Send;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use std::str;

use faad2_sys::{self, NeAACDecHandle, c_char, c_uchar, c_ulong};

unsafe fn static_cstr(ptr: *const c_char) -> &'static str {
    str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
}

pub fn version() -> (&'static str, &'static str) {
    unsafe {
        let mut id: *const c_char = ptr::null();
        let mut copyright: *const c_char = ptr::null();

        faad2_sys::NeAACDecGetVersion(&mut id as *mut _, &mut copyright as *mut _);

        (static_cstr(id), static_cstr(copyright))
    }
}

pub struct Error(c_uchar);

impl Error {
    pub fn message(&self) -> &'static str {
        unsafe {
            static_cstr(faad2_sys::NeAACDecGetErrorMessage(self.0))
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {{ code: {:?}, message: {:?} }}", self.0, self.message())
    }
}

struct DecoderHandle {
    handle: NeAACDecHandle,
}

unsafe impl Send for DecoderHandle {}

impl DecoderHandle {
    fn alloc() -> Self {
        let handle = unsafe { faad2_sys::NeAACDecOpen() };

        if handle == ptr::null_mut() {
            panic!("NeAACDecOpen failed")
        } else {
            DecoderHandle { handle }
        }
    }
}

impl Drop for DecoderHandle {
    fn drop(&mut self) {
        unsafe {
            faad2_sys::NeAACDecClose(self.handle);
        }
    }
}

pub struct Decoder {
    decoder: DecoderHandle,
    sample_rate: usize,
    channels: usize,
}

#[derive(Debug)]
pub struct DecodeResult<'a> {
    pub samples: &'a [f32],
    pub bytes_consumed: usize,
    pub channels: usize,
    pub sample_rate: usize,
}

impl Decoder {
    pub fn new(audio_specific_config: &[u8]) -> Result<Self, ()> {
        unsafe {
            let decoder = DecoderHandle::alloc();

            let mut config = faad2_sys::NeAACDecGetCurrentConfiguration(decoder.handle);
            (*config).outputFormat = faad2_sys::FAAD_FMT_FLOAT;
            if faad2_sys::NeAACDecSetConfiguration(decoder.handle, config) != 1 {
                return Err(());
            }

            let mut sample_rate: c_ulong = 0;
            let mut channels: c_uchar = 0;

            let err = faad2_sys::NeAACDecInit2(
                decoder.handle,
                audio_specific_config.as_ptr(),
                audio_specific_config.len() as c_ulong,
                &mut sample_rate as *mut _,
                &mut channels as *mut _,
            );

            if err != 0 {
                return Err(());
            }

            Ok(Decoder {
                decoder,
                sample_rate: sample_rate as usize,
                channels: channels as usize,
            })
        }
    }

    pub fn sample_rate(&self) -> usize {
        self.sample_rate
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<DecodeResult<'_>, Error> {
        unsafe {
            let mut frame_info = MaybeUninit::zeroed();

            let samples = faad2_sys::NeAACDecDecode(
                self.decoder.handle,
                frame_info.as_mut_ptr(),
                data.as_ptr(),
                data.len() as c_ulong,
            );

            let frame_info = frame_info.assume_init();

            if samples == ptr::null_mut() {
                return Err(Error(frame_info.error));
            }

            let info = DecodeResult {
                samples: slice::from_raw_parts::<f32>(samples as *const f32, frame_info.samples as usize),
                bytes_consumed: frame_info.bytesconsumed as usize,
                channels: frame_info.channels as usize,
                sample_rate: frame_info.samplerate as usize,
            };

            self.channels = info.channels;
            self.sample_rate = info.sample_rate;

            Ok(info)
        }
    }
}
