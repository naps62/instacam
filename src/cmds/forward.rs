use ffmpeg4_ffi::sys;

use crate::av::decoder_ctx::DecoderCtx;
use crate::av::encoder_ctx::EncoderCtx;
use crate::{canvas, opts, pipeline};

pub fn run(args: opts::Forward) {
    unsafe { sys::avdevice_register_all() };

    let mut decoder = DecoderCtx::open(args.input.clone(), &args);
    let mut encoder = EncoderCtx::new(args.output.clone(), &decoder);

    let mut pipeline = pipeline::Pipeline::new(&args, &decoder);

    let canvas = canvas::create(args.clone());

    loop {
        if let Some((_, ref sender)) = canvas {
            sender.send(pipeline.fil_as_msg()).unwrap();
        }

        decoder.decode_frame(pipeline.raw_ref());

        pipeline.process();

        encoder.encode_frame(pipeline.yuv_ref());
    }
}
