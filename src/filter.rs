extern crate opencv;

use ffmpeg4_ffi::sys;
use opencv::core::{BorderTypes, Mat, Point, Size, CV_8UC3};
use opencv::imgproc;

pub fn pixelate(src_frame: *mut sys::AVFrame, dst_frame: *mut sys::AVFrame) {
    let src = unsafe {
        Mat::new_rows_cols_with_data(
            (*src_frame).height,
            (*src_frame).width,
            CV_8UC3,
            (*src_frame).data[0] as *mut std::ffi::c_void,
            (*src_frame).linesize[0] as usize,
        )
        .unwrap()
    };

    let mut dst = unsafe {
        Mat::new_rows_cols_with_data(
            (*dst_frame).height,
            (*dst_frame).width,
            CV_8UC3,
            (*dst_frame).data[0] as *mut std::ffi::c_void,
            (*dst_frame).linesize[0] as usize,
        )
        .unwrap()
    };

    imgproc::blur(
        &src,
        &mut dst,
        Size::new(20, 20),
        Point::new(-1, -1),
        BorderTypes::BORDER_CONSTANT as i32,
    )
    .unwrap();
}
