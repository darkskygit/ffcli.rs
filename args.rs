use crate::utils::{Fail, LogLevel, Packer, StructOpt};
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
    pub fn new(input: String, output: String) -> VideoFilter {
        VideoFilter {
            input,
            output,
            params: Vec::new(),
        }
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
    vf: Option<VideoFilter>,
    filter_complex: Vec<VideoFilter>,
    argv: Packer,
}

impl FFmpegArgs {
    pub fn new() -> FFmpegArgs {
        FFmpegArgs {
            vf: None,
            filter_complex: Vec::new(),
            argv: Packer::from_args(),
        }
    }
    pub fn vf(mut self, vf: VideoFilter) -> FFmpegArgs {
        self.vf = Some(vf);
        self
    }
    pub fn filter(mut self, vf: VideoFilter) -> FFmpegArgs {
        self.filter_complex.push(vf);
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
            Some(FFmpegDefaultArgs::General) => vec!["-stats"],
        });
        let filter_params = match self.vf {
            Some(filter) => vec!["-vf".to_string(), filter.to_string()],
            None => {
                if self.filter_complex.len() > 0 {
                    vec![
                        "-filter_complex".to_string(),
                        self.filter_complex
                            .iter()
                            .map(|arg| arg.to_string())
                            .collect::<Vec<_>>()
                            .join(";"),
                    ]
                } else {
                    vec![]
                }
            }
        };
        args.append(&mut filter_params.iter().map(AsRef::as_ref).collect());
        Ok(args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>())
    }
}
