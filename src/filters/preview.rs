use crossbeam_channel::{Receiver, Sender};
use opencv::imgproc::{self, InterpolationFlags};
use opencv::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::TextureAccess;
use sdl2::video::Window;
use sdl2::EventPump;
use std::slice;
use std::thread;
use std::time::Duration;

use super::{utils, Filter};

use crate::{av, types::Frame, types::FrameMsg};

#[derive(Debug)]
pub struct Preview {
    thread: thread::JoinHandle<()>,
    sender: Sender<FrameMsg>,
    out: Frame,
}

pub fn new(width: i32, height: i32, out: Frame) -> Preview {
    let (sender, receiver) = crossbeam_channel::unbounded::<FrameMsg>();

    let thread = init_window(width, height, receiver);

    Preview {
        thread,
        sender,
        out,
    }
}

impl Filter for Preview {
    fn run(&mut self, src_frame: Frame) -> Frame {
        let src = utils::frame_to_mat(src_frame);
        let mut dst = utils::frame_to_mat(self.out);

        let dst_size = dst.size().unwrap();

        self.sender.send(FrameMsg(src_frame.clone())).unwrap();

        imgproc::resize(
            &src,
            &mut dst,
            dst_size,
            0.0,
            0.0,
            InterpolationFlags::INTER_NEAREST as i32,
        )
        .unwrap();

        self.out
    }

    fn output(&self) -> Frame {
        self.out
    }
}

impl Drop for Preview {
    fn drop(&mut self) {
        av::utils::free_frame(&mut self.out);
    }
}

fn init_window(width: i32, height: i32, receiver: Receiver<FrameMsg>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();
        let window = video
            .window("instacam preview", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut event_pump = ctx.event_pump().unwrap();
        render_loop(window, &mut event_pump, receiver);
    })
}

fn render_loop(window: Window, event_pump: &mut EventPump, receiver: Receiver<FrameMsg>) {
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
