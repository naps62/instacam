extern crate opencv;

pub mod blur;
pub mod pixelate;
mod utils;

use crate::types::Frame;

pub trait Filter {
    fn run(&mut self, src_frame: Frame) -> Frame;
    fn output(&self) -> Frame;
}
