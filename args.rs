use super::{VideoFilter, VideoPreset};
use crate::utils::{LogLevel, ARGS};

pub enum FFmpegDefaultArgs {
    None,
    Quiet,
    General,
}

pub struct FFmpegArgs {
    filters: Vec<VideoFilter>,
    params: Vec<String>,
}

impl FFmpegArgs {
    pub fn new() -> Self {
        FFmpegArgs {
            filters: Vec::new(),
            params: Vec::new(),
        }
    }
    pub fn attach(mut self, format: &str) -> Self {
        self.params
            .append(&mut vec!["-attach".to_string(), format.to_string()]);
        self
    }
    pub fn dump_attachment(mut self, prefix: &str, output: &str) -> Self {
        self.params.append(&mut vec![
            format!("-dump_attachment:{}", prefix).to_string(),
            output.to_string(),
        ]);
        self
    }
    pub fn f(mut self, format: &str) -> Self {
        self.params
            .append(&mut vec!["-f".to_string(), format.to_string()]);
        self
    }
    pub fn fflags(mut self, param: &str) -> Self {
        self.params
            .append(&mut vec!["-fflags".to_string(), param.to_string()]);
        self
    }
    pub fn flags(mut self, prefix: &str, param: &str) -> Self {
        self.params.append(&mut vec![
            format!("-flags:{}", prefix).to_string(),
            param.to_string(),
        ]);
        self
    }
    pub fn framerate(mut self) -> Self {
        self.params.append(&mut vec![
            "-framerate".to_string(),
            ARGS.get_fps().to_string(),
        ]);
        self
    }
    pub fn i(mut self, input: &str) -> Self {
        self.params
            .append(&mut vec!["-i".to_string(), input.to_string()]);
        self
    }
    pub fn map(mut self, map: &str) -> Self {
        self.params
            .append(&mut vec!["-map".to_string(), map.to_string()]);
        self
    }
    pub fn map_metadata(mut self, map_metadata: &str) -> Self {
        self.params.append(&mut vec![
            "-map_metadata".to_string(),
            map_metadata.to_string(),
        ]);
        self
    }
    pub fn metadata(mut self, prefix: &str, param: &str) -> Self {
        self.params.append(&mut vec![
            format!("-metadata:{}", prefix).to_string(),
            param.to_string(),
        ]);
        self
    }
    pub fn preset(mut self, preset: VideoPreset) -> Self {
        self.params
            .append(&mut vec!["-preset".to_string(), preset.to_string()]);
        self
    }
    pub fn profile(mut self, profile: &str) -> Self {
        self.params
            .append(&mut vec!["-profile".to_string(), profile.to_string()]);
        self
    }
    pub fn q(mut self, q: u8) -> Self {
        self.params
            .append(&mut vec!["-q".to_string(), q.to_string()]);
        self
    }
    pub fn raw(mut self, raw_params: &str) -> Self {
        self.params.append(&mut vec![raw_params.to_string()]);
        self
    }
    pub fn raw_str(mut self, raw_str_params: String) -> Self {
        self.params.append(&mut vec![raw_str_params]);
        self
    }
    pub fn vcodec(mut self, vcodec: &str) -> Self {
        self.params
            .append(&mut vec!["-vcodec".to_string(), vcodec.to_string()]);
        self
    }
    pub fn vf(mut self, vf: VideoFilter) -> Self {
        self.filters.push(vf);
        self
    }
    pub fn build_filter(mut self) -> Self {
        self.params.append(&mut match self.filters.len() {
            0 => vec![],
            1 => {
                if self.filters[0].get_inputs().len() > 1 || self.filters[0].get_outputs().len() > 1
                {
                    vec!["-filter_complex".to_string(), self.filters[0].to_string()]
                } else {
                    vec!["-vf".to_string(), self.filters[0].to_string()]
                }
            }
            _ => vec![
                "-filter_complex".to_string(),
                self.filters
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(";"),
            ],
        });
        self.filters.clear();
        self
    }
    pub fn build(self, default: Option<FFmpegDefaultArgs>) -> Vec<String> {
        let mut args = vec!["-hide_banner", "-y"];
        args.append(&mut match default {
            Some(FFmpegDefaultArgs::None) => vec!["-loglevel", "quiet"],
            Some(FFmpegDefaultArgs::Quiet) => vec![
                "-loglevel",
                if ARGS.verbosity.log_level() > LogLevel::Info {
                    "warning"
                } else {
                    "error"
                },
            ],
            Some(FFmpegDefaultArgs::General) => vec![
                "-loglevel",
                if ARGS.verbosity.log_level() > LogLevel::Info {
                    "warning"
                } else {
                    "error"
                },
                "-stats",
            ],
            None => vec![],
        });
        args.append(&mut self.params.iter().map(AsRef::as_ref).collect());
        args.iter().map(ToString::to_string).collect::<Vec<_>>()
    }
}
