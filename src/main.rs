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

    let processor = video_processor::create(app);

    let _ = processor.join();
}
