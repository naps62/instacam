use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
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
        ctx.debug();

        for _i in 0..args.number {
            ctx.read_frame();
            debug_frame(&ctx);
            save_gray_frame(&ctx);
        }
    }
}

pub unsafe fn debug_frame(ctx: &DecoderCtx) {
    let frame = *ctx.frame;
    let codec_ctx = *ctx.codec_ctx;

    let c_type_char = sys::av_get_picture_type_char(frame.pict_type) as u32;

    let type_char = std::char::from_u32(c_type_char).unwrap();

    println!(
        "Frame {:?} (type={} sized={} bytes) pts {} key_frame {} [DTS {}]",
        codec_ctx.frame_number,
        type_char,
        frame.pkt_size,
        frame.pts,
        frame.key_frame,
        frame.coded_picture_number
    )
}

pub unsafe fn save_gray_frame(ctx: &DecoderCtx) {
    let frame = *ctx.frame;
    let number = (*ctx.codec_ctx).frame_number;

    let name = format!("frames/{}.pmg", number);

    let width = frame.width;
    let height = frame.height;
    let linesize = frame.linesize[0];
    let gray_channel = slice::from_raw_parts(frame.data[0], (width * linesize) as usize);

    println!("Saving frame {} into {}", number, name);

    let mut file = File::create(name).unwrap();
    write!(file, "P5\n{} {}\n{}\n", width, height, 255).unwrap();

    for i in 0..height {
        let start = (linesize * i) as usize;
        let end = start + width as usize;

        let line = &gray_channel[start..end];

        file.write_all(line).unwrap();
    }
}
