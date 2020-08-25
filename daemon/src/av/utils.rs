use ffmpeg4_ffi::sys;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null_mut;

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

pub fn av_inv_q(n: sys::AVRational) -> sys::AVRational {
    sys::AVRational {
        num: n.den,
        den: n.num,
    }
}

pub fn alloc_frame(width: i32, height: i32, format: sys::AVPixelFormat) -> *mut sys::AVFrame {
    unsafe {
        let frame = sys::av_frame_alloc();

        (*frame).width = width;
        (*frame).height = height;
        (*frame).format = format;

        sys::av_frame_get_buffer(frame, 0);

        let size = sys::avpicture_get_size(format, width, height);
        let buffer = sys::av_malloc(size as usize);

        sys::avpicture_fill(
            frame as *mut sys::AVPicture,
            buffer as *mut u8,
            format,
            width,
            height,
        );

        (*frame).pts = 0;

        frame
    }
}

pub fn free_frame(frame: &mut *mut sys::AVFrame) {
    unsafe { sys::av_frame_free(frame) };
}

pub fn alloc_sws(
    width: i32,
    height: i32,
    from: sys::AVPixelFormat,
    to: sys::AVPixelFormat,
) -> *mut sys::SwsContext {
    unsafe {
        sys::sws_getContext(
            width,
            height,
            from,
            width,
            height,
            to,
            sys::SWS_BILINEAR as i32,
            null_mut(),
            null_mut(),
            null_mut(),
        )
    }
}

pub fn free_sws(sws: *mut sys::SwsContext) {
    unsafe { sys::sws_freeContext(sws) };
}
