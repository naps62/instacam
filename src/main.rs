extern crate crossbeam_channel;
extern crate sdl2;
extern crate serde;
extern crate serde_json;

mod app;
mod args;
mod av;
mod filters;
mod pipeline;
mod types;
mod video_processor;

fn main() {
    let app = app::new();

    println!("starting processor");
    let (processor, processor_sender) = video_processor::create(app.clone());

    app.lock().unwrap().subscribe(processor_sender);

    let _ = processor.join();
}
