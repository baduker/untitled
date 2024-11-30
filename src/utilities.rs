use std::time;
use chrono::NaiveDate;

pub fn splitter(string: &str, split_on: &str) -> Vec<String> {
    string.split(split_on).map(|s| s.to_string()).collect()
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

pub fn to_snake_case(s: &str) -> String {
    s.replace([' ', '-'], "_").replace('.', "")
}

pub fn format_date(date_str: &str) -> Option<String> {
    NaiveDate::parse_from_str(date_str, "%d %b %Y")
        .ok()
        .map(|date| date.format("%d-%m-%Y").to_string())
}

pub fn today_date() -> String {
    chrono::Local::now().format("%d-%m-%Y").to_string()
}

pub fn validate_id(id: &str) -> bool {
    id.chars().all(char::is_numeric)
}

pub fn format_duration(duration: std::time::Duration) {
    let total_seconds = duration.as_secs();
    if total_seconds >= 60 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        let ms = duration.subsec_millis();
        if ms > 0 {
            format!("Duration: {}m {}s {}ms", minutes, seconds, ms);
        } else {
            format!("Duration: {}m {}s", minutes, seconds);
        }
    } else {
        format!("Duration: {:.2}s", total_seconds);
    }
}
