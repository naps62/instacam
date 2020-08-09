use std::{os, thread};

use crossbeam_channel;
use ffmpeg4_ffi::sys;

use crate::app::Messages;
use crate::{app, av, pipeline};

pub fn create(app: app::App) -> (thread::JoinHandle<()>, crossbeam_channel::Sender<Messages>) {
    let args = app.lock().unwrap().args();
    let settings = app.lock().unwrap().get_settings();
    let (sender, receiver) = crossbeam_channel::unbounded();

    let handle = thread::spawn(move || {
        prepare_libav();

        let mut decoder = av::decoder_ctx::DecoderCtx::open(&args);
        let mut encoder = av::encoder_ctx::EncoderCtx::new(&args, &decoder);

        let mut pipeline = pipeline::Pipeline::new(&args, &settings, &decoder);

        loop {
            // read a new frame from /dev/video0
            decoder.decode_frame(pipeline.raw_ref());

            // do stuff to the frame (blur, etc)
            pipeline.process();

            // write the stuffed frame to /dev/video2
            encoder.encode_frame(pipeline.yuv_ref());

            if let Ok(msg) = receiver.try_recv() {
                match msg {
                    Messages::NewSettings(settings) => {
                        pipeline = pipeline::Pipeline::new(&args, &settings, &decoder)
                    }
                }
            }
        }
    });

    (handle, sender)
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
