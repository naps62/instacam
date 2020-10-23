use opencv::core::{BorderTypes, Point, Size};
use opencv::imgproc;

use super::{utils, Filter};

use crate::{av, types::Frame};

#[derive(Clone, Debug)]
pub struct Blur {
    k: i32,
    out: Frame,
}

pub fn new(k: i32, out: Frame) -> Blur {
    Blur { k, out }
}

impl Filter for Blur {
    fn run(&mut self, src_frame: Frame) -> Frame {
        let src = utils::frame_to_mat(src_frame);
        let mut dst = utils::frame_to_mat(self.out);

        imgproc::blur(
            &src,
            &mut dst,
            Size::new(self.k, self.k),
            Point::new(-1, -1),
            BorderTypes::BORDER_CONSTANT as i32,
        )
        .unwrap();

        self.out
    }

    fn output(&self) -> Frame {
        self.out
    }
}

impl Drop for Blur {
    fn drop(&mut self) {
        av::utils::free_frame(&mut self.out);
    }
}
