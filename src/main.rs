extern crate crossbeam_channel;
extern crate sdl2;

mod av;
mod canvas;
mod cmds;
mod filter;
mod opts;
mod pipeline;
mod types;

use clap::Clap;

use cmds::*;
use opts::SubCommand::*;

fn main() {
    let opts = opts::Opts::parse();

    match opts.subcmd {
        Forward(args) => forward::run(args),
        UI => ui::run(),
    }
}
