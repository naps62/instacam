use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::slice;

use ffmpeg4_ffi::sys;

use crate::av::decoder_ctx::DecoderCtx;
use crate::opts;

pub fn run(args: opts::Photos) {
    unsafe {
        sys::avdevice_register_all();
        sys::avcodec_register_all();
        sys::av_register_all();

        let path = args.input.as_str();
        let format = "v4l2";

        assert!(PathBuf::from(path).exists(), "file {} does not exist", path);

        let mut ctx = DecoderCtx::new(path, format, 640, 480, 20);

        ctx.open_video_stream();

        for i in 0..args.number {
            ctx.read_frame();
            ctx.to_rgb();
            save_frame(&ctx, format!("frames/{}.ppm", i));
        }
    }
}

pub unsafe fn save_frame(ctx: &DecoderCtx, name: String) {
    let codec_ctx = *ctx.codec_ctx;

    let mut file = File::create(name).unwrap();
    write!(
        file,
        "P6\n{} {}\n{}\n",
        codec_ctx.width, codec_ctx.height, 255
    )
    .unwrap();

    let rgb_frame = *ctx.rgb_frame.unwrap();

    let linesize = rgb_frame.linesize[0];
    let data = slice::from_raw_parts(
        rgb_frame.data[0],
        (codec_ctx.height * linesize * 3) as usize,
    );

    for i in 0..codec_ctx.height {
        let start = (linesize * i) as usize;
        let end = start + (codec_ctx.width * 3) as usize;

        let line = &data[start..end];

        file.write_all(line).unwrap();
    }
}
