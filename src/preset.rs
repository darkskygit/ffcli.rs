use failure::Fail;
use std::str::FromStr;

#[derive(Debug, Fail)]
pub enum FFmpegError {
    #[fail(display = "未知的参数预设: {}", name)]
    PresetUnknown { name: String },
}

#[derive(PartialEq, PartialOrd)]
pub enum VideoPreset {
    Placebo,
    VerySlow,
    Slower,
    Slow,
    Medium,
    Fast,
    Faster,
    VeryFast,
    SuperFast,
    UltraFast,
}

impl Default for VideoPreset {
    fn default() -> Self {
        VideoPreset::Placebo
    }
}

impl FromStr for VideoPreset {
    type Err = FFmpegError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "placebo" => Ok(VideoPreset::Placebo),
            "veryslow" => Ok(VideoPreset::VerySlow),
            "slower" => Ok(VideoPreset::Slower),
            "slow" => Ok(VideoPreset::Slow),
            "medium" => Ok(VideoPreset::Medium),
            "fast" => Ok(VideoPreset::Fast),
            "faster" => Ok(VideoPreset::Faster),
            "veryfast" => Ok(VideoPreset::VeryFast),
            "superfast" => Ok(VideoPreset::SuperFast),
            "ultrafast" => Ok(VideoPreset::UltraFast),
            _ => Err(FFmpegError::PresetUnknown {
                name: s.to_string(),
            }),
        }
    }
}

impl ToString for VideoPreset {
    fn to_string(&self) -> String {
        match &self {
            VideoPreset::Placebo => "placebo",
            VideoPreset::VerySlow => "veryslow",
            VideoPreset::Slower => "slower",
            VideoPreset::Slow => "slow",
            VideoPreset::Medium => "medium",
            VideoPreset::Fast => "fast",
            VideoPreset::Faster => "faster",
            VideoPreset::VeryFast => "veryfast",
            VideoPreset::SuperFast => "superfast",
            VideoPreset::UltraFast => "ultrafast",
        }
        .into()
    }
}
