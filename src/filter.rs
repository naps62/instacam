extern crate opencv;

use ffmpeg4_ffi::sys;
use opencv::core::{BorderTypes, Mat, Point, Scalar, Size, CV_8UC3};
use opencv::imgproc::{self, InterpolationFlags, LineTypes};

pub fn pixelate(src_frame: *mut sys::AVFrame, dst_frame: *mut sys::AVFrame) {
    let src = unsafe {
        Mat::new_rows_cols_with_data(
            (*src_frame).width,
            (*src_frame).height,
            CV_8UC3,
            (*src_frame).data[0] as *mut std::ffi::c_void,
            (*src_frame).linesize[0] as usize,
        )
        .unwrap()
    };

    let mut tmp = unsafe {
        Mat::new_size(
            Size::new((*src_frame).width / 16, (*src_frame).height / 16),
            CV_8UC3,
        )
        .unwrap()
    };

    let tmp_size = unsafe { Size::new((*src_frame).width / 16, (*src_frame).height / 16) };

    let mut dst = unsafe {
        Mat::new_rows_cols_with_data(
            (*dst_frame).width,
            (*dst_frame).height,
            CV_8UC3,
            (*dst_frame).data[0] as *mut std::ffi::c_void,
            (*dst_frame).linesize[0] as usize,
        )
        .unwrap()
    };
    let dst_size = unsafe { Size::new((*dst_frame).width, (*dst_frame).height) };

    imgproc::resize(
        &src,
        &mut tmp,
        tmp_size,
        0.0,
        0.0,
        InterpolationFlags::INTER_LINEAR as i32,
    )
    .unwrap();

    println!("{:?} {:?}", tmp_size, dst_size);

    imgproc::blur(
        &src,
        &mut dst,
        Size::new(60, 60),
        Point::new(-1, -1),
        BorderTypes::BORDER_CONSTANT as i32,
    )
    .unwrap();

    // imgproc::resize(
    //     &src,
    //     &mut dst,
    //     Size(),
    //     1.0,
    //     1.0,
    //     2, // InterpolationFlags::WARP_FILL_OUTLIERS as i32,
    // )
    // .unwrap();

    imgproc::circle(
        &mut dst,
        Point::new(200, 200),
        100,
        Scalar::new(255.0, 1.0, 1.0, 0.5),
        10,
        LineTypes::FILLED as i32,
        1,
    )
    .unwrap();
}
