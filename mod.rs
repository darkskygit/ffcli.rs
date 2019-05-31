mod args;
mod filter;
mod preset;

pub use args::{FFmpegArgs, FFmpegDefaultArgs};
pub use filter::{VideoFilter, VideoFilterParams};
pub use preset::VideoPreset;

pub fn info(default: Option<FFmpegDefaultArgs>) -> Vec<String> {
    let filter = VideoFilter::new().input(0).output(1).params("test", "");
    let filter2 = VideoFilter::new()
        .params("alpha", "qwlidouoasd:asdsd")
        .params("sddd".to_string(), "None");
    FFmpegArgs::new()
        .vf(filter)
        // .vf(filter2)
        .build_filter()
        // .vf(filter)
        .vf(filter2)
        .build_filter()
        .build(default)
}
