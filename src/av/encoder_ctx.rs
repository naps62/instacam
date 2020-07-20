use std::ptr::null_mut;

use ffmpeg4_ffi::extra::defs::{averror, averror_eof, eagain};
use ffmpeg4_ffi::sys;

use super::decoder_ctx::DecoderCtx;
use super::utils;
use crate::filter;

#[derive(Clone)]
pub struct EncoderCtx {
    pub av: *mut sys::AVFormatContext,
    pub codec_ctx: *mut sys::AVCodecContext,
    pub codec: *mut sys::AVCodec,
    pub stream: *mut sys::AVStream,
    pub frame: *mut sys::AVFrame,
    pub filtered_frame: *mut sys::AVFrame,
    pub sws_ctx: *mut sys::SwsContext,
    pub sws_ctx2: *mut sys::SwsContext,
    pub frame2: *mut sys::AVFrame,
}

unsafe impl Send for EncoderCtx {}

impl EncoderCtx {
    #[allow(dead_code)]
    pub unsafe fn new(path: &str) -> EncoderCtx {
        let path_str = utils::str_to_c_str(path);

        let mut av: *mut sys::AVFormatContext = null_mut();

        let format = sys::av_guess_format(null_mut(), path_str.as_ptr(), null_mut());

        sys::avformat_alloc_output_context2(&mut av, format, null_mut(), path_str.as_ptr());

        EncoderCtx {
            av: av,
            codec: null_mut(),
            codec_ctx: null_mut(),
            stream: null_mut(),
            frame: null_mut(),
            filtered_frame: null_mut(),
            sws_ctx: null_mut(),
            sws_ctx2: null_mut(),
            frame2: null_mut(),
        }
    }

    pub unsafe fn new_with_format(path: &str, format: &str) -> EncoderCtx {
        let path_str = utils::str_to_c_str(path);
        let format_str = utils::str_to_c_str(format);

        let mut av: *mut sys::AVFormatContext = null_mut();

        let format = sys::av_guess_format(format_str.as_ptr(), null_mut(), null_mut());

        sys::avformat_alloc_output_context2(&mut av, format, null_mut(), path_str.as_ptr());

        EncoderCtx {
            av: av,
            codec: null_mut(),
            codec_ctx: null_mut(),
            stream: null_mut(),
            frame: null_mut(),
            filtered_frame: null_mut(),
            sws_ctx: null_mut(),
            sws_ctx2: null_mut(),
            frame2: null_mut(),
        }
    }

    pub unsafe fn load_stream(&mut self, decoder_ctx: &DecoderCtx, codec_id: u32) {
        self.codec = sys::avcodec_find_encoder(codec_id);
        self.codec_ctx = sys::avcodec_alloc_context3(self.codec);
        self.stream = sys::avformat_new_stream(self.av, null_mut());

        // encoder codec params
        (*self.codec_ctx).height = (*decoder_ctx.codec_ctx).height;
        (*self.codec_ctx).width = (*decoder_ctx.codec_ctx).width;

        (*self.codec_ctx).pix_fmt = sys::AVPixelFormat_AV_PIX_FMT_YUV420P;

        // control rate
        (*self.codec_ctx).bit_rate = 400000;
        // (*self.codec_ctx).sample_fmt = sys::AVSampleFormat_AV_SAMPLE_FMT_S16;
        (*self.codec_ctx).gop_size = 10;
        (*self.codec_ctx).max_b_frames = 1;

        // time base
        let input_framerate =
            sys::av_guess_frame_rate(decoder_ctx.av, decoder_ctx.stream, null_mut());
        let time_base = utils::av_inv_q(input_framerate);
        (*self.codec_ctx).time_base = time_base;
        (*self.stream).time_base = time_base;

        sys::avcodec_open2(self.codec_ctx, self.codec, null_mut());
        sys::avcodec_parameters_from_context((*self.stream).codecpar, self.codec_ctx);
    }

    pub unsafe fn build_frame_context(&mut self, ctx: &DecoderCtx) {
        self.frame = sys::av_frame_alloc();
        self.filtered_frame = sys::av_frame_alloc();
        self.frame2 = sys::av_frame_alloc();
        (*self.frame).width = (*self.codec_ctx).width;
        (*self.frame).height = (*self.codec_ctx).height;
        (*self.frame).format = sys::AVPixelFormat_AV_PIX_FMT_BGR24;
        (*self.filtered_frame).width = (*self.codec_ctx).width;
        (*self.filtered_frame).height = (*self.codec_ctx).height;
        (*self.filtered_frame).format = sys::AVPixelFormat_AV_PIX_FMT_BGR24;
        (*self.frame2).width = (*self.codec_ctx).width;
        (*self.frame2).height = (*self.codec_ctx).height;
        (*self.frame2).format = (*self.codec_ctx).pix_fmt;

        println!(
            "{:?} {:?}",
            (*self.codec_ctx).width,
            (*self.codec_ctx).height
        );
        self.sws_ctx = sys::sws_getContext(
            (*ctx.codec_ctx).width,
            (*ctx.codec_ctx).height,
            (*ctx.codec_ctx).pix_fmt,
            (*self.frame).width,
            (*self.frame).height,
            (*self.frame).format,
            sys::SWS_BILINEAR as i32,
            null_mut(),
            null_mut(),
            null_mut(),
        );

        self.sws_ctx2 = sys::sws_getContext(
            (*self.frame).width,
            (*self.frame).height,
            (*self.frame).format,
            (*self.frame2).width,
            (*self.frame2).height,
            (*self.frame2).format,
            sys::SWS_BILINEAR as i32,
            null_mut(),
            null_mut(),
            null_mut(),
        );

        (*self.frame).pts = 0;
        (*self.frame2).pts = 0;

        sys::av_frame_get_buffer(self.frame, 0);
        sys::av_frame_get_buffer(self.filtered_frame, 0);
        sys::av_frame_get_buffer(self.frame2, 0);

        let size = sys::avpicture_get_size(
            (*self.frame).format,
            (*self.frame).width,
            (*self.frame).height,
        );
        let buffer = sys::av_malloc(size as usize);
        let gray_buffer = sys::av_malloc(size as usize);

        let size2 = sys::avpicture_get_size(
            (*self.frame2).format,
            (*self.frame2).width,
            (*self.frame2).height,
        );
        let buffer2 = sys::av_malloc(size2 as usize);

        sys::avpicture_fill(
            self.frame as *mut sys::AVPicture,
            buffer as *mut u8,
            (*self.frame).format,
            (*self.codec_ctx).width,
            (*self.codec_ctx).height,
        );

        sys::avpicture_fill(
            self.filtered_frame as *mut sys::AVPicture,
            gray_buffer as *mut u8,
            (*self.filtered_frame).format,
            (*self.codec_ctx).width,
            (*self.codec_ctx).height,
        );

        sys::avpicture_fill(
            self.frame2 as *mut sys::AVPicture,
            buffer2 as *mut u8,
            (*self.frame2).format,
            (*self.codec_ctx).width,
            (*self.codec_ctx).height,
        );
    }

    pub unsafe fn open_file(&mut self, path: &str) {
        let path_str = utils::str_to_c_str(path);

        let response = sys::avio_open(
            &mut (*self.av).pb,
            path_str.as_ptr(),
            sys::AVIO_FLAG_WRITE as i32,
        );

        utils::check_error(response);

        let response = sys::avformat_write_header(self.av, null_mut());

        utils::check_error(response);
    }

    pub unsafe fn convert_frame(&mut self, decoder_ctx: &DecoderCtx) {
        sys::sws_scale(
            self.sws_ctx,
            (*decoder_ctx.frame).data.as_ptr() as *const *const u8,
            (*decoder_ctx.frame).linesize.as_ptr() as *const i32,
            0,
            (*decoder_ctx.codec_ctx).height,
            (*self.frame).data.as_ptr() as *const *mut u8,
            (*self.frame).linesize.as_ptr() as *const i32,
        );

        filter::pixelate(self.frame, self.filtered_frame);

        sys::sws_scale(
            self.sws_ctx2,
            (*self.filtered_frame).data.as_ptr() as *const *const u8,
            (*self.filtered_frame).linesize.as_ptr() as *const i32,
            0,
            (*decoder_ctx.codec_ctx).height,
            (*self.frame2).data.as_ptr() as *const *mut u8,
            (*self.frame2).linesize.as_ptr() as *const i32,
        );

        (*self.frame).pts = (*decoder_ctx.frame).pts;
        (*self.frame2).pts = (*decoder_ctx.frame).pts;
    }

    pub unsafe fn encode(&mut self, decoder_ctx: &DecoderCtx) -> i32 {
        let mut packet = sys::av_packet_alloc();

        let mut response = sys::avcodec_send_frame(self.codec_ctx, self.frame2);

        while response >= 0 {
            response = sys::avcodec_receive_packet(self.codec_ctx, packet);

            if response == averror(eagain()) || response == averror_eof() {
                break;
            } else if response < 0 {
                break;
            }

            (*packet).stream_index = 0;

            let out_time = (*self.stream).time_base;
            let frame_rate = (*decoder_ctx.stream).avg_frame_rate;

            (*packet).duration =
                (out_time.den as i64) / (out_time.num as i64) / (frame_rate.num as i64)
                    * (frame_rate.den as i64);
            sys::av_packet_rescale_ts(
                packet,
                (*decoder_ctx.stream).time_base,
                (*self.stream).time_base,
            );
            response = sys::av_interleaved_write_frame(self.av, packet);
        }

        sys::av_packet_unref(packet);
        sys::av_packet_free(&mut packet);

        return 0;
    }
}
