use std::slice;
use std::thread;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::TextureAccess;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::{opts, types};

pub type ThreadSafeFrame = types::FrameMsg;

pub fn create(args: opts::Opts) -> Option<(thread::JoinHandle<()>, Sender<ThreadSafeFrame>)> {
    if args.preview {
        None
    } else {
        let (sender, receiver) = crossbeam_channel::unbounded();

        let handle = thread::spawn(move || {
            let ctx = sdl2::init().unwrap();
            let video_subsystem = ctx.video().unwrap();
            let window = video_subsystem
                .window("instacam", args.width as u32, args.height as u32)
                .position_centered()
                .build()
                .unwrap();

            let mut event_pump = ctx.event_pump().unwrap();

            render_loop(window, &mut event_pump, receiver);
        });

        Some((handle, sender))
    }
}

fn render_loop(window: Window, event_pump: &mut EventPump, receiver: Receiver<ThreadSafeFrame>) {
    let (width, height) = window.size();
    let mut canvas = window.into_canvas().build().unwrap();
    let creator = canvas.texture_creator();

    canvas.set_draw_color(pixels::Color::RGB(0, 255, 255));

    let mut texture = creator
        .create_texture(
            pixels::PixelFormatEnum::BGR24,
            TextureAccess::Streaming,
            width,
            height,
        )
        .unwrap();

    'running: loop {
        let frame_msg = receiver.recv().expect("Failed to receive frame");
        let frame = frame_msg.0;

        let data = unsafe { slice::from_raw_parts((*frame).data[0], (width * height) as usize) };
        let linesize = unsafe { (*frame).linesize[0] };

        texture.update(None, data, linesize as usize).unwrap();

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
