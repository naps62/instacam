use ffmpeg4_ffi::sys;

use std::ptr::null_mut;

use crate::av::decoder_ctx::DecoderCtx;
use crate::{filter, opts, types};

type Frame = *mut sys::AVFrame;

pub struct Pipeline {
    raw: Frame,
    bgr: Frame,
    pub fil: Frame,
    yuv: Frame,
    args: opts::Forward,
    raw2bgr: *mut sys::SwsContext,
    bgr2yuv: *mut sys::SwsContext,
}

const BGR: sys::AVPixelFormat = sys::AVPixelFormat_AV_PIX_FMT_BGR24;
const YUV: sys::AVPixelFormat = sys::AVPixelFormat_AV_PIX_FMT_YUVJ420P;

impl Pipeline {
    pub fn new(args: &opts::Forward, decoder_ctx: &DecoderCtx) -> Pipeline {
        let width = args.width;
        let height = args.height;
        let raw_format = unsafe { (*decoder_ctx.codec_ctx).pix_fmt };

        Pipeline {
            raw: unsafe { sys::av_frame_alloc() },
            bgr: alloc_frame(width, height, BGR),
            fil: alloc_frame(width, height, BGR),
            yuv: alloc_frame(width, height, YUV),
            raw2bgr: sws_alloc(width, height, raw_format, BGR),
            bgr2yuv: sws_alloc(width, height, BGR, YUV),
            args: args.clone(),
        }
    }

    pub fn raw_ref(&self) -> &Frame {
        &self.raw
    }

    pub fn yuv_ref(&self) -> &Frame {
        &self.yuv
    }

    pub fn process(&mut self) {
        sws_convert(self.raw2bgr, self.raw, self.bgr);
        filter::blur(self.bgr, self.fil, self.args.blur);
        sws_convert(self.bgr2yuv, self.fil, self.yuv);
    }

    pub fn fil_as_msg(&self) -> types::FrameMsg {
        types::FrameMsg(self.fil.clone())
    }
}

fn alloc_frame(width: i32, height: i32, format: sys::AVPixelFormat) -> *mut sys::AVFrame {
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

fn sws_convert(ctx: *mut sys::SwsContext, from: *mut sys::AVFrame, to: *mut sys::AVFrame) {
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
