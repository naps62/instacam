use ffmpeg4_ffi::sys;

use crate::av::{self, decoder_ctx::DecoderCtx};
use crate::filters::Filter;
use crate::{app::settings::Settings, args::Args, filters};

type Frame = *mut sys::AVFrame;

pub struct Pipeline {
    raw: Frame,
    bgr: Frame,
    yuv: Frame,
    raw2bgr: *mut sys::SwsContext,
    bgr2yuv: *mut sys::SwsContext,
    #[allow(dead_code)]
    filters: Vec<Box<dyn Filter>>,
}

const BGR: sys::AVPixelFormat = sys::AVPixelFormat_AV_PIX_FMT_BGR24;
const YUV: sys::AVPixelFormat = sys::AVPixelFormat_AV_PIX_FMT_YUVJ420P;

impl Pipeline {
    pub fn new(args: &Args, settings: &Settings, decoder_ctx: &DecoderCtx) -> Pipeline {
        let width = args.width;
        let height = args.height;
        let raw_format = unsafe { (*decoder_ctx.codec_ctx).pix_fmt };

        Pipeline {
            raw: unsafe { sys::av_frame_alloc() },
            bgr: av::utils::alloc_frame(width, height, BGR),
            yuv: av::utils::alloc_frame(width, height, YUV),
            raw2bgr: av::utils::alloc_sws(width, height, raw_format, BGR),
            bgr2yuv: av::utils::alloc_sws(width, height, BGR, YUV),
            filters: alloc_filters(&args, &settings),
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

        let out = self
            .filters
            .iter_mut()
            .fold(self.bgr, |frame, filter| filter.run(frame));

        sws_convert(self.bgr2yuv, out, self.yuv);
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        println!("dropping");
        av::utils::free_frame(&mut self.raw);
        av::utils::free_frame(&mut self.bgr);
        av::utils::free_frame(&mut self.yuv);
        av::utils::free_sws(self.raw2bgr);
        av::utils::free_sws(self.bgr2yuv);
    }
}

pub fn alloc_filters(args: &Args, settings: &Settings) -> Vec<Box<dyn Filter>> {
    use crate::app::settings::Proc::*;
    use filters::*;

    if let Some(procs) = &settings.pipeline {
        procs
            .iter()
            .map(|proc| -> Box<dyn Filter> {
                let frame = av::utils::alloc_frame(args.width, args.height, BGR);

                match proc {
                    Blur { k } => Box::new(blur::new(*k, frame)),
                    Pixelate { k } => Box::new(pixelate::new(*k, frame)),
                    Sepia => Box::new(sepia::new(frame)),
                    Edges { t1, t2 } => Box::new(edges::new(*t1, *t2, frame)),
                    Sharpen => Box::new(sharpen::new(frame)),
                }
            })
            .collect()
    } else {
        vec![]
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
