pub struct FrameMsg(pub *mut ffmpeg4_ffi::sys::AVFrame);
unsafe impl Send for FrameMsg {}
