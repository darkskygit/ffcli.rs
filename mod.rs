mod args;

use args::FFmpegArgs;

pub struct FFmpeg {
    args: FFmpegArgs,
}

impl FFmpeg {
    pub fn new() -> FFmpeg {
        FFmpeg {
            args: FFmpegArgs::new(),
        }
    }
}
