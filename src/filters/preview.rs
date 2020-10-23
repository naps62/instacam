use opencv::imgproc::{self, InterpolationFlags};
use opencv::prelude::*;

use super::{utils, Filter};

use crate::{av, types::Frame};

#[derive(Clone, Debug)]
pub struct Preview {
    out: Frame,
}

pub fn new(out: Frame) -> Preview {
    Preview { out }
}

impl Filter for Preview {
    fn run(&mut self, src_frame: Frame) -> Frame {
        let src = utils::frame_to_mat(src_frame);
        let mut dst = utils::frame_to_mat(self.out);

        let dst_size = dst.size().unwrap();

        imgproc::resize(
            &src,
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

impl Drop for Preview {
    fn drop(&mut self) {
        av::utils::free_frame(&mut self.out);
    }
}
