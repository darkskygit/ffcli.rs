use crate::utils::Fail;
use std::str::FromStr;

#[derive(Debug, Fail)]
pub enum FFmpegError {
    #[fail(display = "未知的参数预设: {}", name)]
    PresetUnknown { name: String },
}

#[derive(Debug, PartialEq)]
pub enum VideoPreset {
    Placebo,
    VerySlow,
    Slower,
    Slow,
    Medium,
    Fast,
    Faster,
    VeryFast,
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
            VideoPreset::Placebo => "placebo".to_owned(),
            VideoPreset::VerySlow => "veryslow".to_owned(),
            VideoPreset::Slower => "slower".to_owned(),
            VideoPreset::Slow => "slow".to_owned(),
            VideoPreset::Medium => "medium".to_owned(),
            VideoPreset::Fast => "fast".to_owned(),
            VideoPreset::Faster => "faster".to_owned(),
            VideoPreset::VeryFast => "veryfast".to_owned(),
            VideoPreset::UltraFast => "ultrafast".to_owned(),
        }
    }
}
