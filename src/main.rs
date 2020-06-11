mod av;
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
        Formats => formats::run(),
        Codecs => codecs::run(),
    }
}
