use std::fmt;

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

impl VideoFilterParams {
    fn new(key: String, value: Option<String>) -> VideoFilterParams {
        VideoFilterParams { key, value }
    }
}

struct VideoFilter {
    input: String,
    output: String,
    params: Vec<VideoFilterParams>,
}

impl VideoFilter {
    fn new() -> VideoFilter {
        VideoFilter {
            input: "".to_string(),
            output: "".to_string(),
            params: Vec::new(),
        }
    }
    fn params(&mut self, key: String, value: Option<String>) -> &VideoFilter {
        self.params.push(VideoFilterParams::new(key, value));
        self
    }
}

impl fmt::Display for VideoFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (has_in, has_out, has_params) = (
            self.input.len() > 0,
            self.output.len() > 0,
            self.params.len() == 0,
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

pub struct FFmpegArgs {
    vf: VideoFilter,
    filter_complex: Vec<VideoFilter>,
}

impl FFmpegArgs {
    pub fn new() -> FFmpegArgs {
        FFmpegArgs {
            vf: VideoFilter::new(),
            filter_complex: Vec::new(),
        }
    }
}
