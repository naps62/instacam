extern crate opencv;

use opencv::core::{BorderTypes, Mat, Point, Size, CV_8UC3};
use opencv::imgproc::{self, InterpolationFlags};
use opencv::prelude::*;

use crate::types::Frame;

pub fn blur(src_frame: Frame, dst_frame: Frame, ksize: i32) {
    let src = frame_to_mat(src_frame);
    let mut dst = frame_to_mat(dst_frame);

    imgproc::blur(
        &src,
        &mut dst,
        Size::new(ksize, ksize),
        Point::new(-1, -1),
        BorderTypes::BORDER_CONSTANT as i32,
    )
    .unwrap();
}

pub fn pixelate(src_frame: Frame, dst_frame: Frame, ksize: i32) {
    let src = frame_to_mat(src_frame);
    let mut dst = frame_to_mat(dst_frame);

    let dst_size = dst.size().unwrap();

    let mut tmp = unsafe { Mat::new_size(src.size().unwrap() / 32, CV_8UC3).unwrap() };
    let tmp_size = tmp.size().unwrap();

    imgproc::resize(
        &src,
        &mut tmp,
        tmp_size,
        0.0,
        0.0,
        InterpolationFlags::INTER_LINEAR as i32,
    )
    .unwrap();

    imgproc::resize(
        &tmp,
        &mut dst,
        dst_size,
        0.0,
        0.0,
        InterpolationFlags::INTER_NEAREST as i32,
    )
    .unwrap();
}

fn frame_to_mat(frame: Frame) -> Mat {
    unsafe {
        Mat::new_rows_cols_with_data(
            (*frame).height,
            (*frame).width,
            CV_8UC3,
            (*frame).data[0] as *mut std::ffi::c_void,
            (*frame).linesize[0] as usize,
        )
        .unwrap()
    }
}
