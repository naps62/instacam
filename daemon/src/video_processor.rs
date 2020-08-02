use std::{os, thread};

use ffmpeg4_ffi::sys;

use crate::{app, av, canvas, pipeline};

pub fn create(app: app::App) -> thread::JoinHandle<()> {
    let opts = app.lock().unwrap().opts();

    thread::spawn(move || {
        prepare_libav();

        let mut decoder = av::decoder_ctx::DecoderCtx::open(&opts);
        let mut encoder = av::encoder_ctx::EncoderCtx::new(&opts, &decoder);

        let mut pipeline = pipeline::Pipeline::new(&opts, &decoder);

        let canvas = canvas::create(opts.clone());

        loop {
            // read a new frame from /dev/video0
            decoder.decode_frame(pipeline.raw_ref());

            // do stuff to the frame (blur, etc)
            pipeline.process();

            // write the stuffed frame to /dev/video2
            encoder.encode_frame(pipeline.yuv_ref());

            // if preview is enabled, update it
            if let Some((_, ref sender)) = canvas {
                sender.send(pipeline.fil_as_msg()).unwrap();
            }
        }
    })
}

fn prepare_libav() {
    unsafe {
        sys::avdevice_register_all();
        sys::av_log_set_callback(None);
    };
}

#[no_mangle]
#[allow(dead_code)]
extern "C" fn av_log_callback(
    _av_class: *mut core::ffi::c_void,
    level: os::raw::c_int,
    format: *const os::raw::c_char,
    args: *mut sys::__va_list_tag,
) {
    use std::ffi::CStr;

    if level > 15 {
        return;
    }

    unsafe {
        println!("{:?} {:?} {:?}", level, CStr::from_ptr(format), args);
    }
}
