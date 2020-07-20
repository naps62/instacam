extern crate clap;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Miguel Palhas <mpalhas@gmail.com")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Forward(Forward),
    UI,
}

#[derive(Clap)]
pub struct Forward {
    #[clap(short = "i", long = "input")]
    pub input: String,
    #[clap(short = "o", long = "output")]
    pub output: String,
    #[clap(short = "p", long = "preview")]
    pub preview: bool,
}
