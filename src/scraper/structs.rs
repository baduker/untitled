use serde::{Deserialize, Serialize};

fn is_false(b: &bool) -> bool {
    // This means "not the value of b"; * is the dereference operator and ! is the logical NOT operator.
    !*b
}

#[derive(Debug)]
pub struct Selectors;

impl Selectors {
    /// The set of CSS selectors to scrape the kindgirls.com website.
    pub const MODEL_INFO: &'static str = r#"#model_info"#;
    pub const MODEL_GALLERIES: &'static str = r#".gal_list a"#;
    pub const MODEL_VIDEOS: &'static str = r#".video_list a"#;
}

#[derive(Serialize, Deserialize)]
struct Girl {
    /// This is the main container for a model from the kindgirls.com website.
    #[serde(rename = "isSingleGallery", skip_serializing_if = "is_false")]
    is_single_gallery: bool,

    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    bio: Option<Bio>,

    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    content: Option<Visuals>,

    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    stats: Option<Stats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Bio {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,

    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub(crate) country: Option<String>,

    #[serde(rename = "birth_year", skip_serializing_if = "Option::is_none")]
    pub(crate) birth_year: Option<String>,

    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub(crate) alias: Option<Vec<String>>,
}

impl Bio {
    pub fn new(info: Vec<String>) -> Self {
        let mut bio = Bio {
            name: None,
            country: None,
            birth_year: None,
            alias: None,
        };

        for (index, item) in info.iter().enumerate() {
            match index {
                0 => bio.name = Some(item.clone()),
                1 => bio.country = Some(item.clone()),
                2 => bio.birth_year = Some(item.clone()),
                3 => bio.alias = Some(Self::parse_alias(item)),
                _ => break,
            }
        }
        bio
    }

    fn parse_alias(alias: &str) -> Vec<String> {
        alias
            .strip_prefix("Alias: ")
            .unwrap_or(alias)
            .split(",")
            .map(|s| s.trim().to_string())
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Gallery {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<String>,

    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub(crate) date: Option<String>,

    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub(crate) link: Option<String>,

    #[serde(rename = "total_photos", skip_serializing_if = "Option::is_none")]
    pub(crate) total_photos: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct Visuals {
    #[serde(rename = "thumb", skip_serializing_if = "Option::is_none")]
    thumb_nail: Option<String>,

    #[serde(rename = "galleries", skip_serializing_if = "Option::is_none")]
    galleries: Option<Vec<Gallery>>,

    #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
    videos: Option<Vec<Video>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Video {
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub(crate) link: Option<String>,

    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub(crate) source: Option<String>,

    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct Stats {
    #[serde(rename = "total_galleries", skip_serializing_if = "Option::is_none")]
    total_galleries: Option<i32>,

    #[serde(rename = "total_photos", skip_serializing_if = "Option::is_none")]
    total_photos: Option<i32>,

    #[serde(rename = "total_videos", skip_serializing_if = "Option::is_none")]
    total_videos: Option<i32>,
}
