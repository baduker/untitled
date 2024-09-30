use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::config::Config;

pub fn splitter(string: &str, split_on: &str) -> Vec<String> {
    string.split(split_on).map(|s| s.to_string()).collect()
}

pub fn create_dirs<T: Config>(config: &T, girl: &Girl) -> Result<(), Box<dyn Error>> {
    let base_dir = PathBuf::from(config.download_dir());
    let girl_name = girl.bio.get_name().to_string();

    let mut paths_to_create = vec![
        base_dir.join(&girl_name),
        base_dir.join(&girl_name).join("videos"),
    ];

    // Add gallery paths
    for gallery in &girl.content.galleries {
        if let Some(date) = &gallery.date {
            paths_to_create.push(base_dir.join(&girl_name).join(date));
        }
    }

    // Create all directories
    for path in paths_to_create {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
    }

    Ok(())
}

pub fn build_video_src_url(source: String) -> String {
    let base_suffix = ".mp4";
    let source_base_url = "https://vids.kindgirls.com/d9";
    /*
    This does a couple of things:
    - it takes this - /vids/scbig/mila-azul-3.jpg, which is the source
    - splits on /
    - removes the .jpg or jpeg suffix
    - adds .mp4 suffix instead
    - glues this all together to build an actual video source url

    For example:
    from this

        /vids/scbig/mila-azul-3.jp

    to this

        https://vids.kindgirls.com/d9/mila-azul-3.mp4

    */
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
