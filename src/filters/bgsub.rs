use opencv::core::{Mat, CV_8UC3};
use opencv::prelude::*;
use opencv::video::prelude::BackgroundSubtractorKNN;
use opencv::{core, video};
// use opencv::bgsegm;

use super::{utils, Filter};

use crate::{av, types::Frame};

pub struct BgSub {
    subtractor: core::Ptr<dyn video::BackgroundSubtractorKNN>,
    fg_mask: Mat,
    out: Frame,
}

pub fn new(out: Frame) -> BgSub {
    let out_size = utils::frame_to_mat(out).size().unwrap();
    let subtractor = video::create_background_subtractor_knn(500, 400.0, true).unwrap();
    let fg_mask = unsafe { Mat::new_size(out_size, CV_8UC3).unwrap() };

    BgSub {
        subtractor,
        out,
        fg_mask,
    }
}

impl Filter for BgSub {
    fn run(&mut self, src_frame: Frame) -> Frame {
        let src = utils::frame_to_mat(src_frame);
        let mut dst = utils::frame_to_mat(self.out);

        BackgroundSubtractorKNN::apply(&mut self.subtractor, &src, &mut self.fg_mask, -1.0)
            .unwrap();
        // BackgroundSubtractorKNN::apply(&mut self.subtractor, &src, &mut self.fg_mask, -1.0)
        //     .unwrap();
        dst.set_to(&core::Scalar::all(0.0), &core::no_array().unwrap())
            .unwrap();
        core::copy_to(&src, &mut dst, &self.fg_mask).unwrap();

        self.out
    }

    fn output(&self) -> Frame {
        self.out
    }
}

impl Drop for BgSub {
    fn drop(&mut self) {
        av::utils::free_frame(&mut self.out);
    }
}

impl Clone for BgSub {
    fn clone(&self) -> Self {
        new(self.out)
    }
}
