pub type Frame = *mut ffmpeg4_ffi::sys::AVFrame;

pub struct FrameMsg(pub Frame);
unsafe impl Send for FrameMsg {}
