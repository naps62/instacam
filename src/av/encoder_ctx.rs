use std::ptr::null_mut;

use ffmpeg4_ffi::extra::defs::{averror, averror_eof, eagain};
use ffmpeg4_ffi::sys;

use super::decoder_ctx::DecoderCtx;
use super::utils;

use crate::opts;

pub struct EncoderCtx {
    pub av: *mut sys::AVFormatContext,
    pub codec_ctx: *mut sys::AVCodecContext,
    pub stream: *mut sys::AVStream,
    pub path: String,
    packet: *mut sys::AVPacket,
}

unsafe impl Send for EncoderCtx {}

const CODEC_ID_RAW: u32 = sys::AVCodecID_AV_CODEC_ID_RAWVIDEO;

impl EncoderCtx {
    pub fn new(args: &opts::Opts, decoder_ctx: &DecoderCtx) -> EncoderCtx {
        unsafe {
            let path = args.output.clone();

            let path_str = utils::str_to_c_str(path.as_str());
            let format_str = utils::str_to_c_str("v4l2");

            let mut av: *mut sys::AVFormatContext = null_mut();

            let format = sys::av_guess_format(format_str.as_ptr(), null_mut(), null_mut());

            sys::avformat_alloc_output_context2(&mut av, format, null_mut(), path_str.as_ptr());

            let codec = sys::avcodec_find_encoder(CODEC_ID_RAW);
            let codec_ctx = sys::avcodec_alloc_context3(codec);
            let stream = sys::avformat_new_stream(av, null_mut());

            // encoder codec params
            (*codec_ctx).height = (*decoder_ctx.codec_ctx).height;
            (*codec_ctx).width = (*decoder_ctx.codec_ctx).width;

            (*codec_ctx).pix_fmt = sys::AVPixelFormat_AV_PIX_FMT_YUV420P;

            // control rate
            (*codec_ctx).bit_rate = 400000;
            (*codec_ctx).gop_size = 10;
            (*codec_ctx).max_b_frames = 1;

            // time base
            let input_framerate =
                sys::av_guess_frame_rate(decoder_ctx.av, decoder_ctx.stream, null_mut());
            let time_base = utils::av_inv_q(input_framerate);
            (*codec_ctx).time_base = time_base;
            (*stream).time_base = time_base;

            sys::avcodec_open2(codec_ctx, codec, null_mut());
            sys::avcodec_parameters_from_context((*stream).codecpar, codec_ctx);

            let response = sys::avio_open(
                &mut (*av).pb,
                path_str.as_ptr(),
                sys::AVIO_FLAG_WRITE as i32,
            );

            utils::check_error(response);

            let response = sys::avformat_write_header(av, null_mut());

            utils::check_error(response);

            EncoderCtx {
                av: av,
                codec_ctx: codec_ctx,
                stream: stream,
                path: path,
                packet: sys::av_packet_alloc(),
            }
        }
    }

    pub fn encode_frame(&mut self, frame: &*mut sys::AVFrame) -> i32 {
        unsafe {
            let mut response = sys::avcodec_send_frame(self.codec_ctx, *frame);

            while response >= 0 {
                response = sys::avcodec_receive_packet(self.codec_ctx, self.packet);

                if response == averror(eagain()) || response == averror_eof() {
                    break;
                } else if response < 0 {
                    break;
                }

                response = sys::av_interleaved_write_frame(self.av, self.packet);
            }

            sys::av_packet_unref(self.packet);
        }

        return 0;
    }
}
