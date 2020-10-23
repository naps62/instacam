use opencv::core::{self, Mat};

use super::{utils, Filter};

use crate::{av, types::Frame};

#[derive(Clone, Debug)]
pub struct Sepia {
    kernel: Mat,
    out: Frame,
}

pub fn new(out: Frame) -> Sepia {
    let kernel = Mat::from_slice_2d(&vec![
        vec![0.272, 0.534, 0.131],
        vec![0.349, 0.686, 0.168],
        vec![0.393, 0.769, 0.189],
    ])
    .unwrap();

    Sepia { kernel, out }
}

impl Filter for Sepia {
    fn run(&mut self, src_frame: Frame) -> Frame {
        let src = utils::frame_to_mat(src_frame);
        let mut dst = utils::frame_to_mat(self.out);

        core::transform(&src, &mut dst, &self.kernel).unwrap();

        self.out
    }

    fn output(&self) -> Frame {
        self.out
    }
}

impl Drop for Sepia {
    fn drop(&mut self) {
        av::utils::free_frame(&mut self.out);
    }
}
