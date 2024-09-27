use std::error::Error;
use std::fs;
use std::path::Path;

use crate::config::Config;

pub fn splitter(string: &str, split_on: &str) -> Vec<String> {
    string.split(split_on).map(|s| s.to_string()).collect()
}

pub fn create_dir<T: Config>(config: &T, dir_name: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(dir_name).exists() {
        fs::create_dir(dir_name)?;
    }
    Ok(())
}

pub fn build_video_src_url(source: String) -> String {
    let base_suffix = ".mp4";
    let source_base_url = "https://vids.kindgirls.com/d9";
    // /vids/scbig/mila-azul-3.jpg
    let video_name = source
        .split("/")
        .last()
        .unwrap_or("")
        .strip_suffix(".jpg")
        .or_else(|| source.strip_suffix(".jpeg"))
        .unwrap_or("");
    let video_file = format!("{}{}", video_name, base_suffix);
    format!("{}/{}", source_base_url, video_file)
}

pub fn parse_video_duration(duration: &str) -> u32 {
    // Split video duration in the MM:SS format and get total seconds
    duration
        .split(":")
        .collect::<Vec<&str>>()
        .as_slice()
        .try_into()
        .map(|[minutes, seconds]: [&str; 2]| {
            let minutes: u32 = minutes.parse().unwrap_or(0);
            let seconds: u32 = seconds.parse().unwrap_or(0);
            minutes * 60 + seconds
        })
        .unwrap_or_else(|_| 0)
}
