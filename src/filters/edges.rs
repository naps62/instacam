use opencv::core::{Mat, CV_8UC1};
use opencv::imgproc;
use opencv::prelude::*;

use super::{utils, Filter};

use crate::{av, types::Frame};

#[derive(Clone, Debug)]
pub struct Edges {
    t1: f64,
    t2: f64,
    edges: Mat,
    out: Frame,
}

pub fn new(t1: f64, t2: f64, out: Frame) -> Edges {
    let out_size = utils::frame_to_mat(out).size().unwrap();
    let edges = unsafe { Mat::new_size(out_size, CV_8UC1).unwrap() };

    Edges { t1, t2, edges, out }
}

impl Filter for Edges {
    fn run(&mut self, src_frame: Frame) -> Frame {
        let src = utils::frame_to_mat(src_frame);
        let mut dst = utils::frame_to_mat(self.out);

        imgproc::canny(&src, &mut self.edges, self.t1, self.t2, 3, false).unwrap();
        imgproc::cvt_color(&self.edges, &mut dst, imgproc::COLOR_GRAY2BGR, 3).unwrap();

        self.out
    }

    fn output(&self) -> Frame {
        self.out
    }
}

impl Drop for Edges {
    fn drop(&mut self) {
        av::utils::free_frame(&mut self.out);
    }
}
