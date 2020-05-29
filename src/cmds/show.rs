extern crate sdl2;

use std::path::PathBuf;
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
        let format = "v4l2";

        assert!(PathBuf::from(path).exists(), "file {} does not exist", path);

        let mut ctx = DecoderCtx::new(path, format, 1024, 768, 20);
        ctx.open_video_stream();

        let sdl_ctx = sdl2::init().unwrap();
        let video_sybsystem = sdl_ctx.video().unwrap();
        let window = video_sybsystem
            .window("instacam", 1280, 1024)
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

        let width = (*ctx.codec_ctx).width as u32;
        let height = (*ctx.codec_ctx).height as u32;
        let mut texture = creator
            .create_texture(
                pixels::PixelFormatEnum::RGB24,
                TextureAccess::Streaming,
                width,
                height,
            )
            .unwrap();

        let frame = *ctx.rgb_frame;
        let data =
            slice::from_raw_parts(frame.data[0], frame.linesize[0] as usize * height as usize);

        'running: loop {
            i = (i + 1) % 255;
            ctx.read_frame();
            ctx.to_rgb();
            texture
                .update(None, data, frame.linesize[0] as usize)
                .unwrap();

            // canvas.set_draw_color(pixels::Color::RGB(i, 64, 255 - i));
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
