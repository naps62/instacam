use opencv::core::{Mat, Size, BORDER_DEFAULT, CV_8UC3};
use opencv::imgproc::{self, InterpolationFlags};
use opencv::prelude::*;

use super::{utils, Filter};

use crate::{av, types::Frame};

pub struct BgSub2 {
    gaussian_k: i32,
    blurred: Mat,
    out: Frame, // subtractor: core::Ptr<dyn video::BackgroundSubtractorKNN>,
                // fg_mask: Mat,
                // out: Frame
}

pub fn new(out: Frame) -> BgSub2 {
    // let out_size = utils::frame_to_mat(out).size().unwrap();
    // let subtractor = video::create_background_subtractor_knn(500, 400.0, true).unwrap();
    // let fg_mask = unsafe { Mat::new_size(out_size, CV_8UC3).unwrap() };
    let blurred =
        unsafe { Mat::new_size(Size::new((*out).width, (*out).height), CV_8UC3).unwrap() };

    BgSub2 {
        gaussian_k: 5,
        blurred,
        out,
        // subtractor,
        // out,
        // fg_mask,
    }
}

impl Filter for BgSub2 {
    fn run(&mut self, src_frame: Frame) -> Frame {
        self.blur(src_frame);

        self.copy();

        self.out
    }

    fn output(&self) -> Frame {
        self.out
    }
}

impl BgSub2 {
    fn blur(&mut self, src_frame: Frame) {
        let src = utils::frame_to_mat(src_frame);

        imgproc::gaussian_blur(
            &src,
            &mut self.blurred,
            Size::new(self.gaussian_k, self.gaussian_k),
            0.0,
            0.0,
            BORDER_DEFAULT as i32,
        )
        .unwrap();
    }

    fn copy(&mut self) {
        let mut dst = utils::frame_to_mat(self.out);
        let dst_size = dst.size().unwrap();

        imgproc::resize(
            &self.blurred,
            &mut dst,
            dst_size,
            0.0,
            0.0,
            InterpolationFlags::INTER_NEAREST as i32,
        )
        .unwrap();
    }
}

impl Drop for BgSub2 {
    fn drop(&mut self) {
        av::utils::free_frame(&mut self.out);
    }
}

impl Clone for BgSub2 {
    fn clone(&self) -> Self {
        new(self.out)
    }
}
