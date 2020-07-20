mod av;
mod canvas;
mod cmds;
mod opts;

use clap::Clap;

use cmds::*;
use opts::SubCommand::*;

fn main() {
    let opts = opts::Opts::parse();

    match opts.subcmd {
        Photos(args) => photos::run(args),
        Show(args) => show::run(args),
        Record(args) => record::run(args),
        Forward(args) => forward::run(args),
        UI => ui::run(),
        Formats => formats::run(),
        Codecs => codecs::run(),
    }
}
