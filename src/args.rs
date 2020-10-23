extern crate clap;

use clap::Clap;

#[derive(Clap, Clone)]
#[clap(version = "1.0", author = "Miguel Palhas <mpalhas@gmail.com")]
pub struct Args {
    #[clap(short = 'i', long = "input")]
    pub input: String,

    #[clap(short = 'o', long = "output")]
    pub output: String,

    #[clap(short = 'w', long = "width", default_value = "640")]
    pub width: i32,

    #[clap(short = 'h', long = "height", default_value = "480")]
    pub height: i32,

    #[clap(short = 'b', long = "blur", default_value = "20")]
    pub blur: i32,

    #[clap(short = 'f', long = "fps", default_value = "20")]
    pub fps: i64,
}
