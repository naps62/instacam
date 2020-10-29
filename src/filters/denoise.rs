use opencv::photo;

use super::{utils, Filter};

use crate::{av, types::Frame};

#[derive(Clone, Debug)]
pub struct Denoise {
    out: Frame,
}

pub fn new(out: Frame) -> Denoise {
    Denoise { out }
}

impl Filter for Denoise {
    fn run(&mut self, src_frame: Frame) -> Frame {
        let src = utils::frame_to_mat(src_frame);
        let mut dst = utils::frame_to_mat(self.out);

        photo::fast_nl_means_denoising_colored(&src, &mut dst, 3.0, 3.0, 7, 3).unwrap();

        self.out
    }

    fn output(&self) -> Frame {
        self.out
    }
}

impl Drop for Denoise {
    fn drop(&mut self) {
        av::utils::free_frame(&mut self.out);
    }
}
