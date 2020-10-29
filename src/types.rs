pub type Frame = *mut ffmpeg4_ffi::sys::AVFrame;
pub struct FrameMsg(pub *mut ffmpeg4_ffi::sys::AVFrame);
unsafe impl Send for FrameMsg {}
