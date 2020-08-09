use opencv::core::{BorderTypes, Mat, Point};
use opencv::imgproc;

use super::{utils, Filter};

use crate::types::Frame;

#[derive(Clone, Debug)]
pub struct Sharpen {
    kernel: Mat,
    out: Frame,
}

pub fn new(out: Frame) -> Sharpen {
    let kernel =
        Mat::from_slice_2d(&vec![vec![-1, -1, -1], vec![-1, 9, -1], vec![-1, -1, -1]]).unwrap();

    Sharpen { kernel, out }
}

impl Filter for Sharpen {
    fn run(&mut self, src_frame: Frame) -> Frame {
        let src = utils::frame_to_mat(src_frame);
        let mut dst = utils::frame_to_mat(self.out);

        imgproc::filter_2d(
            &src,
            &mut dst,
            -1,
            &self.kernel,
            Point::new(-1, -1),
            0.0,
            BorderTypes::BORDER_REPLICATE as i32,
        )
        .unwrap();

        self.out
    }

    fn output(&self) -> Frame {
        self.out
    }
}
