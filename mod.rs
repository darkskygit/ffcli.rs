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
    let result = FFmpegArgs::new().filter(filter).build(default)?;
    Ok(result)
}
