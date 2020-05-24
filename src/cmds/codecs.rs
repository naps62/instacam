use std::ptr::null_mut;

use ffmpeg4_ffi::sys;

use crate::av::utils;

pub fn run() {
    unsafe {
        sys::av_register_all();

        let mut codec = null_mut();

        loop {
            codec = sys::av_codec_next(codec);

            if codec == null_mut() {
                break;
            }

            let name = utils::c_str_to_string((*codec).name);
            let long_name = utils::c_str_to_string((*codec).long_name);
            println!("{:-20} {}", name, long_name);
        }
    }
}
