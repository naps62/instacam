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
    let codec_ctx = *ctx.codec_ctx;
    let number = (*ctx.codec_ctx).frame_number;

    let name = format!("frames/{}.pmg", number);

    let width = frame.width;
    let height = frame.height;

    let format = sys::AVPixelFormat_AV_PIX_FMT_RGB24;
    let num_bytes = sys::avpicture_get_size(format, codec_ctx.width, codec_ctx.height) as usize;
    let buffer = vec![0u8; num_bytes];

    let sws_ctx = sys::sws_getContext(
        codec_ctx.width,
        codec_ctx.height,
        codec_ctx.pix_fmt,
        codec_ctx.width,
        codec_ctx.height,
        format,
        sys::SWS_BILINEAR as i32,
        null_mut(),
        null_mut(),
        null_mut(),
    );
    let rgb_frame = sys::av_frame_alloc();

    sys::avpicture_fill(
        rgb_frame as *mut sys::AVPicture,
        buffer.as_ptr(),
        format,
        codec_ctx.width,
        codec_ctx.height,
    );

    sys::sws_scale(
        sws_ctx,
        frame.data.as_ptr() as *const *const u8,
        frame.linesize.as_ptr() as *const i32,
        0,
        codec_ctx.height,
        (*rgb_frame).data.as_ptr() as *const *mut u8,
        (*rgb_frame).linesize.as_ptr() as *const i32,
    );

    let linesize = (*rgb_frame).linesize[0];

    println!(
        "width: {}\nheight: {}\nlinesize: {}\nheight*linesize: {}\nbyte size: {}",
        width,
        height,
        linesize,
        height * linesize,
        num_bytes
    );
    println!("Saving frame {} into {}", number, name);
    println!("{:?}", (*rgb_frame).linesize);

    let mut file = File::create(name).unwrap();
    write!(file, "P6\n{} {}\n{}\n", width, height, 255).unwrap();

    let data = slice::from_raw_parts((*rgb_frame).data[0], (height * linesize * 3) as usize);

    for i in 0..height {
        let start = (linesize * i) as usize;
        let end = start + (width * 3) as usize;

        let line = &data[start..end];

        file.write_all(line).unwrap();
    }
}
