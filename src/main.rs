extern crate crossbeam_channel;
extern crate sdl2;

mod av;
mod canvas;
mod filter;
mod opts;
mod pipeline;
mod types;

use clap::Clap;
use ffmpeg4_ffi::sys;

fn main() {
    let args = opts::Opts::parse();

    unsafe { sys::avdevice_register_all() };

    let mut decoder = av::decoder_ctx::DecoderCtx::open(&args);
    let mut encoder = av::encoder_ctx::EncoderCtx::new(&args, &decoder);

    let mut pipeline = pipeline::Pipeline::new(&args, &decoder);

    let canvas = canvas::create(args.clone());

    loop {
        // if preview is enabled, update it
        if let Some((_, ref sender)) = canvas {
            sender.send(pipeline.fil_as_msg()).unwrap();
        }

        // read a new frame from /dev/video0
        decoder.decode_frame(pipeline.raw_ref());

        // do stuff to the frame (blur, etc)
        pipeline.process();

        // write the stuffed frame to /dev/video2
        encoder.encode_frame(pipeline.yuv_ref());
    }
}
