use crate::utils::{Fail, LogLevel, Packer, StructOpt, SubPacker};
use std::fmt;

#[derive(Debug, Fail)]
pub enum ArgsError {
    #[fail(display = "未知错误: {}", name)]
    UnknownError { name: String },
}

pub type ArgsResult<T> = Result<T, ArgsError>;

struct VideoFilterParams {
    key: String,
    value: Option<String>,
}

impl fmt::Display for VideoFilterParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(val) => write!(f, "{}={}", self.key, val),
            None => write!(f, "{}", self.key),
        }
    }
}

pub struct VideoFilter {
    input: String,
    output: String,
    params: Vec<VideoFilterParams>,
}

impl VideoFilter {
    pub fn new_str(input: &str, output: &str) -> VideoFilter {
        VideoFilter::new(input.to_string(), output.to_string())
    }
    pub fn new(input: String, output: String) -> VideoFilter {
        VideoFilter {
            input,
            output,
            params: Vec::new(),
        }
    }
    pub fn params_str(self, key: &str, value: &str) -> VideoFilter {
        let val_str = value.to_string();
        self.params(
            key.to_string(),
            if val_str.len() > 0 {
                Some(val_str)
            } else {
                None
            },
        )
    }
    pub fn params(mut self, key: String, value: std::option::Option<String>) -> VideoFilter {
        self.params.push(VideoFilterParams { key, value });
        self
    }
}

impl fmt::Display for VideoFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (has_in, has_out, has_params) = (
            self.input.len() > 0,
            self.output.len() > 0,
            self.params.len() > 0,
        );
        let filter = if has_params {
            self.params
                .iter()
                .map(|filter| filter.to_string())
                .collect::<Vec<_>>()
                .join(";")
        } else {
            "nullsink".to_string()
        };
        if has_in && has_out && has_params {
            write!(f, "[{}]{}[{}]", self.input, filter, self.output)
        } else if has_in && !has_out {
            write!(f, "[{}]{}", self.input, filter)
        } else if has_out && !has_in {
            write!(f, "{}[{}]", filter, self.output)
        } else if has_params {
            write!(f, "{}", filter)
        } else {
            write!(f, "")
        }
    }
}

pub enum FFmpegDefaultArgs {
    None,
    Quiet,
    General,
}

pub struct FFmpegArgs {
    argv: Packer,
    filters: Vec<VideoFilter>,
    params: Vec<String>,
}

impl FFmpegArgs {
    pub fn new() -> FFmpegArgs {
        FFmpegArgs {
            argv: Packer::from_args(),
            filters: Vec::new(),
            params: Vec::new(),
        }
    }
    pub fn f(mut self, format: &str) -> FFmpegArgs {
        self.params
            .append(&mut vec!["-f".to_string(), format.to_string()]);
        self
    }
    pub fn framerate(mut self) -> FFmpegArgs {
        self.params.append(&mut vec![
            "-framerate".to_string(),
            match self.argv.cmd {
                SubPacker::Compress { fps, .. } => fps,
                _ => 60,
            }
            .to_string(),
        ]);
        self
    }
    pub fn i(mut self, input: &str) -> FFmpegArgs {
        self.params
            .append(&mut vec!["-i".to_string(), input.to_string()]);
        self
    }
    pub fn map(mut self, map: &str) -> FFmpegArgs {
        self.params
            .append(&mut vec!["-map".to_string(), map.to_string()]);
        self
    }
    pub fn raw(mut self, raw_params: &str) -> FFmpegArgs {
        self.params.append(&mut vec![raw_params.to_string()]);
        self
    }
    pub fn raw_str(mut self, raw_str_params: String) -> FFmpegArgs {
        self.params.append(&mut vec![raw_str_params]);
        self
    }
    pub fn vf(mut self, vf: VideoFilter) -> FFmpegArgs {
        self.filters.push(vf);
        self
    }
    pub fn build_filter(mut self) -> FFmpegArgs {
        self.params.append(&mut match self.filters.len() {
            0 => vec![],
            1 => vec!["-vf".to_string(), self.filters[0].to_string()],
            _ => vec![
                "-filter_complex".to_string(),
                self.filters
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(";"),
            ],
        });
        self.filters.clear();
        self
    }
    pub fn build(self, default: Option<FFmpegDefaultArgs>) -> ArgsResult<Vec<String>> {
        let mut args = vec!["-hide_banner", "-y"];
        args.append(&mut match default {
            None | Some(FFmpegDefaultArgs::None) => vec!["-loglevel", "quiet"],
            Some(FFmpegDefaultArgs::Quiet) => vec![
                "-loglevel",
                if self.argv.verbosity.log_level() > LogLevel::Info {
                    "warning"
                } else {
                    "error"
                },
            ],
            Some(FFmpegDefaultArgs::General) => vec![
                "-loglevel",
                if self.argv.verbosity.log_level() > LogLevel::Info {
                    "warning"
                } else {
                    "error"
                },
                "-stats",
            ],
        });
        args.append(&mut self.params.iter().map(AsRef::as_ref).collect());
        Ok(args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>())
    }
}
