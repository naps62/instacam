extern crate crossbeam_channel;
extern crate sdl2;

use std::path::PathBuf;
use std::time::Instant;

use ffmpeg4_ffi::sys;

use crate::av::decoder_ctx::DecoderCtx;
use crate::av::encoder_ctx::EncoderCtx;
use crate::{canvas, opts, pipeline};

pub fn run(args: opts::Forward) {
    unsafe { sys::avdevice_register_all() };

    let mut decoder = DecoderCtx::open(args.input.clone(), &args);
    let raw_pix_fmt = unsafe { (*decoder.codec_ctx).pix_fmt };

    let mut encoder = EncoderCtx::new(args.output.clone(), "v4l2");
    encoder.load_stream(&decoder, sys::AVCodecID_AV_CODEC_ID_RAWVIDEO);
    encoder.open_file();

    let (sender, receiver) = crossbeam_channel::unbounded();

    if args.preview {
        canvas::create(args.width, args.height, receiver);
    }

    let mut pipeline = pipeline::Pipeline::new(&args, raw_pix_fmt);

    loop {
        println!("{:?}", raw_pix_fmt);
        if args.preview {
            sender.send(pipeline.fil_as_msg()).unwrap();
        }

        println!("reading frame");
        let now = Instant::now();
        decoder.read_frame(pipeline.raw_ref());
        println!("read frame: {}", now.elapsed().as_millis());

        pipeline.process();
        encoder.encode(&decoder, pipeline.yuv_ref());
    }
}
