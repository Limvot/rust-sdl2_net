extern crate libc;
// export it for now

use libc::{c_int, c_char, c_void};
use std::ffi::CString;

pub mod ffi;


pub fn init() -> () { unsafe { ffi::SDLNet_Init(); } }
pub fn quit() -> () { unsafe { ffi::SDLNet_Quit(); } }
pub fn get_error() -> String {
    unsafe {
        let raw = &ffi::SDLNet_GetError();
        std::str::from_utf8(std::ffi::c_str_to_bytes(raw)).unwrap().to_string()
    }
}

