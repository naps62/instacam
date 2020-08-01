extern crate crossbeam_channel;
extern crate sdl2;

use clap::Clap;
use ffmpeg4_ffi::sys;
use instacam::{av, canvas, opts, pipeline};

fn main() {
    let args = opts::Opts::parse();

    unsafe { sys::avdevice_register_all() };

    let mut decoder = av::decoder_ctx::DecoderCtx::open(&args);
    let mut encoder = av::encoder_ctx::EncoderCtx::new(&args, &decoder);

    let mut pipeline = pipeline::Pipeline::new(&args, &decoder);

    let canvas = canvas::create(args.clone());

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
}
