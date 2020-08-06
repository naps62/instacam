#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;

extern crate crossbeam_channel;
extern crate sdl2;
extern crate serde;
extern crate serde_json;

mod app;
mod av;
mod canvas;
mod filters;
mod opts;
mod pipeline;
mod server;
mod types;
mod video_processor;

fn main() {
    let app = app::new();

    println!("starting server");
    let _server = server::create(app.clone());
    println!("starting processor");
    let (processor, processor_sender) = video_processor::create(app.clone());

    app.lock().unwrap().subscribe(processor_sender);

    let _ = processor.join();
}
