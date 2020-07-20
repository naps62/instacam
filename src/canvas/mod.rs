use std::slice;
use std::thread;
use std::time::Duration;

use crossbeam_channel::Receiver;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::TextureAccess;
use sdl2::video::Window;
use sdl2::EventPump;

use std::sync::{Arc, Mutex};

use crate::av::encoder_ctx::EncoderCtx;

type ThreadSafeFrame = Arc<Mutex<EncoderCtx>>;

pub fn create(receiver: Receiver<ThreadSafeFrame>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let ctx = sdl2::init().unwrap();
        let video_subsystem = ctx.video().unwrap();
        let window = video_subsystem
            .window("instacam", 1280, 720)
            .position_centered()
            .build()
            .unwrap();

        let mut event_pump = ctx.event_pump().unwrap();

        render_loop(window, &mut event_pump, receiver);
    })
}

fn render_loop(window: Window, event_pump: &mut EventPump, receiver: Receiver<ThreadSafeFrame>) {
    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    canvas.set_draw_color(pixels::Color::RGB(0, 255, 255));

    let width: usize = 1280;
    let height: usize = 720;

    let mut texture = creator
        .create_texture(
            pixels::PixelFormatEnum::RGB24,
            TextureAccess::Streaming,
            width as u32,
            height as u32,
        )
        .unwrap();

    'running: loop {
        let encoder_ctx = receiver.recv().expect("Failed to receive frame");
        let frame = encoder_ctx.lock().unwrap().frame;

        let data = unsafe { slice::from_raw_parts((*frame).data[0], width * height) };
        // let y_plane = unsafe { slice::from_raw_parts((*frame).data[0], width * height) };
        // let u_plane = unsafe { slice::from_raw_parts((*frame).data[1], width * height / 4) };
        // let v_plane = unsafe { slice::from_raw_parts((*frame).data[2], width * height / 4) };
        // let uv_pitch = width / 2;
        let linesize = unsafe { (*frame).linesize[0] };

        texture.update(None, data, linesize as usize).unwrap();
        // .update_yuv(None, y_plane, width, u_plane, uv_pitch, v_plane, uv_pitch)
        // .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

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

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
