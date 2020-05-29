use ffmpeg4_ffi::sys;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub unsafe fn averror_to_str(error: i32) -> String {
    let c_str = sys::strerror(error);

    c_str_to_string(c_str)
}

pub unsafe fn c_str_to_string(c_str: *const c_char) -> String {
    CStr::from_ptr(c_str).to_str().unwrap().to_string()
}

pub fn str_to_c_str(str: &str) -> CString {
    CString::new(str).expect("could not alloc CString")
}

pub fn string_to_c_str(str: String) -> CString {
    CString::new(&str[..]).expect("could not alloc CString")
}

pub fn check_error(response: i32) -> bool {
    if response < 0 {
        println!("error: {}", unsafe { averror_to_str(response) });
    }

    response < 0
}
