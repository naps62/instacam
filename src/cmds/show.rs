extern crate sdl2;

use std::path::PathBuf;
use std::ptr::null_mut;
use std::slice;
use std::time::Duration;

use ffmpeg4_ffi::sys;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::TextureAccess;

use crate::av::decoder_ctx::DecoderCtx;
use crate::opts;

pub fn run(args: opts::Show) {
    unsafe {
        sys::avdevice_register_all();

        let path = args.input.as_str();

        assert!(PathBuf::from(path).exists(), "file {} does not exist", path);

        let mut ctx = DecoderCtx::new(path);
        ctx.open_video_stream();

        let sdl_ctx = sdl2::init().unwrap();
        let video_subsystem = sdl_ctx.video().unwrap();
        let window = video_subsystem
            .window("instacam", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let creator = canvas.texture_creator();

        canvas.set_draw_color(pixels::Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_ctx.event_pump().unwrap();
        let mut i = 0;

        let width = (*ctx.codec_ctx).width as usize;
        let height = (*ctx.codec_ctx).height as usize;
        let mut texture = creator
            .create_texture(
                pixels::PixelFormatEnum::YV12,
                TextureAccess::Streaming,
                width as u32,
                height as u32,
            )
            .unwrap();

        let (sws_ctx, yuv_frame) = build_yuv_context(&ctx);

        let y_plane = slice::from_raw_parts((*yuv_frame).data[0], width * height);
        let u_plane = slice::from_raw_parts((*yuv_frame).data[1], width * height / 4);
        let v_plane = slice::from_raw_parts((*yuv_frame).data[2], width * height / 4);
        let uv_pitch = width / 2;

        'running: loop {
            i = (i + 1) % 255;
            ctx.read_frame();
            to_yuv(&ctx, sws_ctx, yuv_frame);

            texture
                .update_yuv(None, y_plane, width, u_plane, uv_pitch, v_plane, uv_pitch)
                .unwrap();

            canvas.clear();
            canvas.copy(&texture, None, None).unwrap();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

unsafe fn build_yuv_context(ctx: &DecoderCtx) -> (*mut sys::SwsContext, *mut sys::AVFrame) {
    let frame = sys::av_frame_alloc();
    (*frame).width = (*ctx.codec_ctx).width;
    (*frame).height = (*ctx.codec_ctx).width;
    (*frame).format = sys::AVPixelFormat_AV_PIX_FMT_YUV420P;
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

unsafe fn to_yuv(ctx: &DecoderCtx, sws_ctx: *mut sys::SwsContext, frame: *mut sys::AVFrame) {
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
