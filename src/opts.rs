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
    Photos(Photos),
    Show(Show),
    Formats,
    Codecs,
}

#[derive(Clap)]
pub struct Photos {
    #[clap(short = "i", long = "input")]
    pub input: String,
    #[clap(short = "n", long = "number", default_value = "1")]
    pub number: i32,
}

#[derive(Clap)]
pub struct Show {
    #[clap(short = "i", long = "input")]
    pub input: String,
}
