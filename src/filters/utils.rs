use opencv::core::{Mat, CV_8UC3};

use crate::types::Frame;

pub fn frame_to_mat(frame: Frame) -> Mat {
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
