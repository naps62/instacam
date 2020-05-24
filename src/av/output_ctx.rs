use std::ptr::null_mut;

use ffmpeg4_ffi::sys;

use super::utils;

pub struct OutputCtx {
    pub av: *mut sys::AVFormatContext,
}

impl OutputCtx {
    pub unsafe fn new(path: &str) -> OutputCtx {
        let path_str = utils::str_to_c_str(path);

        let mut av: *mut sys::AVFormatContext = null_mut();

        sys::avformat_alloc_output_context2(&mut av, null_mut(), null_mut(), path_str.as_ptr());

        OutputCtx { av: av }
    }

    pub unsafe fn open_file(&mut self, path: &str) {
        let path_str = utils::str_to_c_str(path);

        let response = sys::avio_open(
            &mut (*self.av).pb,
            path_str.as_ptr(),
            sys::AVIO_FLAG_WRITE as i32,
        );

        utils::check_error(response);

        let response = sys::avformat_write_header(self.av, null_mut());

        utils::check_error(response);
    }
}
