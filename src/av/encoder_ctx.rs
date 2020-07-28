use std::ptr::null_mut;

use ffmpeg4_ffi::extra::defs::{averror, averror_eof, eagain};
use ffmpeg4_ffi::sys;

use super::decoder_ctx::DecoderCtx;
use super::utils;

pub struct EncoderCtx {
    pub av: *mut sys::AVFormatContext,
    pub codec_ctx: *mut sys::AVCodecContext,
    pub codec: *mut sys::AVCodec,
    pub stream: *mut sys::AVStream,
    pub sws_yuv2bgr: *mut sys::SwsContext,
    pub sws_bgr2yuv: *mut sys::SwsContext,
}

unsafe impl Send for EncoderCtx {}

impl EncoderCtx {
    pub unsafe fn new(path: &str, format: &str) -> EncoderCtx {
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
            sws_yuv2bgr: null_mut(),
            sws_bgr2yuv: null_mut(),
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

    pub unsafe fn encode(&mut self, decoder_ctx: &DecoderCtx, frame: &*mut sys::AVFrame) -> i32 {
        let mut packet = sys::av_packet_alloc();

        let mut response = sys::avcodec_send_frame(self.codec_ctx, *frame);

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
