use std::env;

fn main() {
    for (name, _value) in env::vars() {
        if name.starts_with("DEP_FFMPEG_") {
            let feature_name = name["DEP_FFMPEG_".len()..name.len()].to_lowercase();
            if feature_name == "ff_api_xvmc" {
                // disable XVMC for ffmpeg 5.1
                continue;
            }
            println!(r#"cargo:rustc-cfg=feature="{}""#, feature_name);
        }
    }
}
