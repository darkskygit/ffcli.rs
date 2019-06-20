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

    fn append_params<T>(mut self, param: T) -> Self
    where
        T: ToString,
    {
        self.params.push(param.to_string());
        self
    }

    pub fn attach<T>(self, format: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-attach").append_params(format)
    }

    pub fn c<F, P>(self, format: F, prefix: P) -> Self
    where
        F: ToString,
        P: ToString,
    {
        let prefix = prefix.to_string();
        if !prefix.is_empty() {
            self.append_params(format!("-c:{}", prefix))
        } else {
            self.append_params("-c")
        }
        .append_params(format)
    }
    pub fn dump_attachment<P, O>(self, prefix: P, output: O) -> Self
    where
        P: ToString,
        O: ToString,
    {
        self.append_params(format!("-dump_attachment:{}", prefix.to_string()))
            .append_params(output)
    }
    pub fn f<T>(self, format: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-f").append_params(format)
    }
    pub fn fflags<T>(self, param: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-fflags").append_params(param)
    }
    pub fn flags<P1, P2>(self, prefix: P1, param: P2) -> Self
    where
        P1: ToString,
        P2: ToString,
    {
        self.append_params(format!("-flags:{}", prefix.to_string()))
            .append_params(param)
    }
    pub fn framerate<T>(self, fps: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-framerate").append_params(fps)
    }
    pub fn i<T>(self, input: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-i").append_params(input)
    }
    pub fn map<T>(self, map: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-map").append_params(map)
    }
    pub fn map_metadata<T>(self, map_metadata: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-map_metadata")
            .append_params(map_metadata)
    }
    pub fn metadata<P1, P2>(self, prefix: P1, param: P2) -> Self
    where
        P1: ToString,
        P2: ToString,
    {
        self.append_params(format!("-metadata:{}", prefix.to_string()))
            .append_params(param)
    }
    pub fn profile<T>(self, profile: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-profile").append_params(profile)
    }
    pub fn q<T>(self, q: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-q").append_params(q)
    }
    pub fn raw<T>(self, raw_param: T) -> Self
    where
        T: ToString,
    {
        self.append_params(raw_param)
    }
    pub fn vcodec<T>(self, vcodec: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-vcodec").append_params(vcodec)
    }
    pub fn vsync<T>(self, param: T) -> Self
    where
        T: ToString,
    {
        self.append_params("-vsync").append_params(param)
    }

    pub fn preset(self, preset: VideoPreset) -> Self {
        self.append_params("-preset").append_params(preset)
    }

    pub fn vf(mut self, vf: VideoFilter) -> Self {
        self.filters.push(vf);
        self
    }

    fn clear_filter(mut self) -> Self {
        self.filters.clear();
        self
    }

    pub fn build_filter(self) -> Self {
        if !self.filters.is_empty() {
            let filters = &self.filters.clone();
            match self.filters.len() {
                1 => if self.filters[0].get_inputs().len() > 1
                    || self.filters[0].get_outputs().len() > 1
                {
                    self.append_params("-filter_complex")
                } else {
                    self.append_params("-vf")
                }
                .append_params(filters[0].clone()),
                _ => self.append_params("-filter_complex").append_params(
                    filters
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(";"),
                ),
            }
            .clear_filter()
        } else {
            self
        }
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
