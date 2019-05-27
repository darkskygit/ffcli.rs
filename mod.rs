mod args;

pub use args::FFmpegDefaultArgs;
use args::{FFmpegArgs, VideoFilter};

pub fn info(default: Option<FFmpegDefaultArgs>) -> Vec<String> {
    let filter =
        VideoFilter::new("0".to_string(), "1".to_string()).params("test".to_string(), None);
    let filter2 = VideoFilter::new_str("", "")
        .params_str("alpha", "qwlidouoasd:asdsd")
        .params("sddd".to_string(), None);
    FFmpegArgs::new()
        .vf(filter)
        // .vf(filter2)
        .build_filter()
        // .vf(filter)
        .vf(filter2)
        .build_filter()
        .build(default)
}

pub fn pack(output: String) -> Vec<String> {
    FFmpegArgs::new()
        .framerate()
        .f("image2pipe")
        .i("pipe:")
        .map("[out]")
        .raw("vcodec")
        .raw_str(output)
        .build(Some(FFmpegDefaultArgs::General))
}
