use opencv::core::{Mat, Size, CV_8UC3};
use opencv::imgproc::{self, InterpolationFlags};
use opencv::prelude::*;

use super::{utils, Filter};

use crate::types::Frame;

#[derive(Clone, Debug)]
pub struct Pixelate {
    k: i32,
    tmp: Mat,
    out: Frame,
}

pub fn new(k: i32, out: Frame) -> Pixelate {
    let tmp = unsafe { Mat::new_size(Size::new(k, k), CV_8UC3).unwrap() };

    Pixelate { k, tmp, out }
}

impl Filter for Pixelate {
    fn run(&mut self, src_frame: Frame) -> Frame {
        let src = utils::frame_to_mat(src_frame);
        let mut dst = utils::frame_to_mat(self.out);

        let dst_size = dst.size().unwrap();
        let tmp_size = self.tmp.size().unwrap();

        imgproc::resize(
            &src,
            &mut self.tmp,
            tmp_size,
            0.0,
            0.0,
            InterpolationFlags::INTER_LINEAR as i32,
        )
        .unwrap();

        imgproc::resize(
            &self.tmp,
            &mut dst,
            dst_size,
            0.0,
            0.0,
            InterpolationFlags::INTER_NEAREST as i32,
        )
        .unwrap();

        self.out
    }

    fn output(&self) -> Frame {
        self.out
    }
}
