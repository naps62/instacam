#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;

extern crate crossbeam_channel;
extern crate sdl2;

mod app;
mod av;
mod canvas;
mod filter;
mod opts;
mod pipeline;
mod server;
mod types;
mod video_processor;

fn main() {
    let app = app::new();

    let _server = server::create(app.clone());
    let processor = video_processor::create(app.clone());

    let _ = processor.join();
}
