use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::slice;
use std::thread::sleep;
use std::time::Duration;

use ffmpeg4_ffi::sys;

use crate::av::decoder_ctx::DecoderCtx;
use crate::opts;

pub fn run(args: opts::Photos) {
    unsafe {
        sys::avdevice_register_all();

        let path = args.input.as_str();
        let format = "v4l2";

        assert!(PathBuf::from(path).exists(), "file {} does not exist", path);

        let mut ctx = DecoderCtx::new(path, format, 640, 480, 1);

        ctx.open_video_stream();

        let (sws_ctx, rgb_frame) = build_rgb_context(&ctx);

        for i in 0..args.number {
            ctx.read_frame();
            to_rgb(&ctx, sws_ctx, rgb_frame);
            save_frame(&ctx, rgb_frame, format!("frames/{}.ppm", i));
            sleep(Duration::from_secs(1))
        }
    }
}

unsafe fn build_rgb_context(ctx: &DecoderCtx) -> (*mut sys::SwsContext, *mut sys::AVFrame) {
    let frame = sys::av_frame_alloc();
    (*frame).width = (*ctx.codec_ctx).width;
    (*frame).height = (*ctx.codec_ctx).width;
    (*frame).format = sys::AVPixelFormat_AV_PIX_FMT_RGB24;
    sys::av_frame_get_buffer(frame, 0);
    let sws_ctx = sys::sws_getContext(
        (*ctx.codec_ctx).width,
        (*ctx.codec_ctx).height,
        (*ctx.codec_ctx).pix_fmt,
        (*ctx.codec_ctx).width,
        (*ctx.codec_ctx).height,
        (*frame).format,
        sys::SWS_BILINEAR as i32,
        null_mut(),
        null_mut(),
        null_mut(),
    );

    sys::avpicture_fill(
        frame as *mut sys::AVPicture,
        (*frame).data[0],
        (*frame).format,
        (*ctx.codec_ctx).width,
        (*ctx.codec_ctx).height,
    );

    (sws_ctx, frame)
}

unsafe fn to_rgb(ctx: &DecoderCtx, sws_ctx: *mut sys::SwsContext, frame: *mut sys::AVFrame) {
    sys::sws_scale(
        sws_ctx,
        (*ctx.frame).data.as_ptr() as *const *const u8,
        (*ctx.frame).linesize.as_ptr() as *const i32,
        0,
        (*ctx.codec_ctx).height,
        (*frame).data.as_ptr() as *const *mut u8,
        (*frame).linesize.as_ptr() as *const i32,
    );
}

pub unsafe fn save_frame(ctx: &DecoderCtx, frame: *mut sys::AVFrame, name: String) {
    let codec_ctx = *ctx.codec_ctx;

    let mut file = File::create(name).unwrap();
    write!(
        file,
        "P6\n{} {}\n{}\n",
        codec_ctx.width, codec_ctx.height, 255
    )
    .unwrap();

    let linesize = (*frame).linesize[0];
    let data = slice::from_raw_parts((*frame).data[0], (codec_ctx.height * linesize * 3) as usize);

    for i in 0..codec_ctx.height {
        let start = (linesize * i) as usize;
        let end = start + (codec_ctx.width * 3) as usize;

        let line = &data[start..end];

        file.write_all(line).unwrap();
    }
}
