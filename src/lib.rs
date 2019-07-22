mod args;
mod filter;
mod preset;

pub use args::{FFmpegArgs, FFmpegDefaultArgs};
pub use filter::{VideoFilter, VideoFilterParams};
pub use preset::VideoPreset;

#[test]
fn test() {
    use log::Level;
    let filter_split = VideoFilter::new()
        .input(0)
        .output("rgb")
        .output("alpha")
        .params("format", "rgb32")
        .params("split", 2);
    let filter_rgb = VideoFilter::new()
        .input("rgb")
        .output("out0")
        .params_raw(
            VideoFilterParams::new()
                .key("lutrgb")
                .params_raw(VideoFilterParams::kv("a", "minval")),
        )
        .params("format", "rgb24");
    let filter_alpha = VideoFilter::new()
        .input("alpha")
        .output("out1")
        .params_key("alphaextract")
        .params("format", "gray");
    let args = FFmpegArgs::new(Level::Debug)
        .i("test.png")
        .vf(filter_split)
        .vf(filter_rgb)
        .vf(filter_alpha)
        .build_filter()
        .map("[out0]")
        .raw("rgb.png")
        .map("[out1]")
        .raw("alpha.png")
        .build();
    assert_eq!("-hide_banner -y -loglevel warning -stats -i test.png -filter_complex [0]format=rgb32,split=2[rgb][alpha];[rgb]lutrgb=a='minval',format=rgb24[out0];[alpha]alphaextract,format=gray[out1] -map [out0] rgb.png -map [out1] alpha.png", args.join(" "));
}