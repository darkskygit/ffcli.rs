mod args;

use crate::utils::{Fail, Packer};
pub use args::FFmpegDefaultArgs;
use args::{ArgsError, FFmpegArgs, VideoFilter};

#[derive(Debug, Fail)]
pub enum FFmpegError {
    #[fail(display = "参数错误: {}", name)]
    ArgsError { name: String },
}

pub type FFmpegResult<T> = Result<T, FFmpegError>;

impl From<ArgsError> for FFmpegError {
    fn from(error: ArgsError) -> Self {
        FFmpegError::ArgsError {
            name: error.to_string(),
        }
    }
}

pub fn info(default: Option<FFmpegDefaultArgs>) -> FFmpegResult<Vec<String>> {
    let filter =
        VideoFilter::new("0".to_string(), "1".to_string()).params("test".to_string(), None);
    let filter2 = VideoFilter::new_str("", "")
        .params_str("alpha", "qwlidouoasd:asdsd")
        .params("sddd".to_string(), None);
    let result = FFmpegArgs::new()
        .vf(filter)
        // .vf(filter2)
        .build_filter()
        // .vf(filter)
        .vf(filter2)
        .build_filter()
        .build(default)?;
    Ok(result)
}

pub fn pack(output: String) -> FFmpegResult<Vec<String>> {
    let result = FFmpegArgs::new()
        .framerate()
        .f("image2pipe")
        .i("pipe:")
        .map("[out]")
        .raw("vcodec")
        .raw_str(output)
        .build(Some(FFmpegDefaultArgs::General))?;
    Ok(result)
}
