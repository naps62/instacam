extern crate crossbeam_channel;
extern crate sdl2;

use std::path::PathBuf;
use std::ptr::null_mut;

use ffmpeg4_ffi::sys;

use crate::av::decoder_ctx::DecoderCtx;
use crate::av::encoder_ctx::EncoderCtx;
use crate::canvas;
use crate::filter;
use crate::opts;
use std::sync::{Arc, Mutex};

const BGR: sys::AVPixelFormat = sys::AVPixelFormat_AV_PIX_FMT_BGR24;
const YUV: sys::AVPixelFormat = sys::AVPixelFormat_AV_PIX_FMT_YUV420P;

pub fn run(args: opts::Forward) {
    let width = args.width;
    let height = args.height;

    unsafe { sys::avdevice_register_all() };

    let input_path = args.input.as_str();
    let output_path = args.output.as_str();

    assert!(
        PathBuf::from(input_path).exists(),
        "file {} does not exist",
        input_path
    );

    let mut decoder = DecoderCtx::open(input_path, 30, width, height);

    let mut encoder = EncoderCtx::new(output_path, "v4l2");
    encoder.load_stream(&decoder, sys::AVCodecID_AV_CODEC_ID_RAWVIDEO);
    encoder.open_file(output_path);

    let (sender, receiver) = crossbeam_channel::unbounded();

    if args.preview {
        canvas::create(width, height, receiver);
    }

    let frame_raw = unsafe { sys::av_frame_alloc() };
    let frame_bgr = alloc_frame(width, height, BGR);
    let frame_fil = alloc_frame(width, height, BGR);
    let frame_yuv = alloc_frame(width, height, YUV);

    let yuv2bgr = sws_alloc(width, height, unsafe { (*decoder.codec_ctx).pix_fmt }, BGR);
    let bgr2yuv = sws_alloc(width, height, BGR, YUV);

    let msg = Arc::new(Mutex::new(canvas::FrameMsg(frame_fil)));

    loop {
        if args.preview {
            sender.send(msg.clone()).unwrap();
        }

        decoder.read_frame(&frame_raw);
        sws_convert(yuv2bgr, frame_raw, frame_bgr);
        filter::blur(frame_bgr, frame_fil, args.blur);
        sws_convert(bgr2yuv, frame_fil, frame_yuv);
        encoder.encode(&decoder, &frame_yuv);
    }
}

pub fn alloc_frame(width: i32, height: i32, format: sys::AVPixelFormat) -> *mut sys::AVFrame {
    unsafe {
        let frame = sys::av_frame_alloc();

        (*frame).width = width;
        (*frame).height = height;
        (*frame).format = format;

        sys::av_frame_get_buffer(frame, 0);

        let size = sys::avpicture_get_size(format, width, height);
        let buffer = sys::av_malloc(size as usize);

        sys::avpicture_fill(
            frame as *mut sys::AVPicture,
            buffer as *mut u8,
            format,
            width,
            height,
        );

        (*frame).pts = 0;

        frame
    }
}

pub fn sws_alloc(
    width: i32,
    height: i32,
    from: sys::AVPixelFormat,
    to: sys::AVPixelFormat,
) -> *mut sys::SwsContext {
    unsafe {
        sys::sws_getContext(
            width,
            height,
            from,
            width,
            height,
            to,
            sys::SWS_BILINEAR as i32,
            null_mut(),
            null_mut(),
            null_mut(),
        )
    }
}

pub fn sws_convert(ctx: *mut sys::SwsContext, from: *mut sys::AVFrame, to: *mut sys::AVFrame) {
    unsafe {
        sys::sws_scale(
            ctx,
            (*from).data.as_ptr() as *const *const u8,
            (*from).linesize.as_ptr() as *const i32,
            0,
            (*from).height,
            (*to).data.as_ptr() as *const *mut u8,
            (*to).linesize.as_ptr() as *const i32,
        );

        (*to).pts = (*from).pts;
    };
}
