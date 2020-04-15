use std::os::raw::{c_int, c_char};

extern "C" {
    pub fn NeAACDecGetVersion(id: *mut *const c_char, copyright: *mut *const c_char) -> c_int;
}
