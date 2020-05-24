use ffmpeg4_ffi::sys;

use super::decoder_ctx::DecoderCtx;
use super::utils::c_str_to_string;

pub unsafe fn debug_decoder_ctx(ctx: &DecoderCtx) {
    let av = *ctx.av;

    let name = c_str_to_string((*av.iformat).long_name);
    let streams = ctx.get_streams();

    println!("Format {} | # Streams {}", name, streams.len());

    for (i, stream_ptr) in streams.iter().enumerate() {
        println!("\nStream #{}:", i);

        let stream = *stream_ptr;
        let codec_params_ptr = (*stream).codecpar;
        let codec_params = *codec_params_ptr;

        let codec = sys::avcodec_find_decoder(codec_params.codec_id);
        let codec_name = c_str_to_string((*codec).long_name);

        println!(
            "Codec {}, ID {}, bit rate {}",
            codec_name,
            (*codec).id,
            codec_params.bit_rate
        );
        match codec_params.codec_type {
            sys::AVMediaType_AVMEDIA_TYPE_VIDEO => {
                println!(
                    "Video codec: resolution {} x {}",
                    codec_params.width, codec_params.height
                );
            }
            sys::AVMediaType_AVMEDIA_TYPE_AUDIO => {
                println!(
                    "Audio codec: {} channels, sample rate {}",
                    codec_params.channels, codec_params.sample_rate
                );
            }
            sys::AVMediaType_AVMEDIA_TYPE_SUBTITLE => {
                println!("Subtitles track");
            }

            x => unreachable!("Found unexpected codec type {:?}", x),
        }
    }
}
