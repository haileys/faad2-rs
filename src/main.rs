use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use libfaad2_sys;

fn main() {
    let mut id: *const c_char = ptr::null();
    let mut copyright: *const c_char = ptr::null();
    unsafe { libfaad2_sys::NeAACDecGetVersion(&mut id as *mut *const c_char, &mut copyright as *mut *const c_char); }
    let id = unsafe { CStr::from_ptr(id) }.to_string_lossy();
    let copyright = unsafe { CStr::from_ptr(copyright) }.to_string_lossy();
    println!("{} {}", id, copyright);
}
