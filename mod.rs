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
    let filter2 = VideoFilter::new("asd".to_string(), "asdwwed".to_string())
        .params("alpha".to_string(), Some("qwlidouoasd:asdsd".to_string()))
        .params("sddd".to_string(), None);
    let result = FFmpegArgs::new()
        .filter(filter)
        .filter(filter2)
        .build(default)?;
    Ok(result)
}
