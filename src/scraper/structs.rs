use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub struct Selectors;

impl Selectors {
    /// The set of CSS selectors to scrape the kindgirls.com website.
    pub const MODEL_INFO: &'static str = r#"#model_info"#;
    pub const MODEL_GALLERIES: &'static str = r#".gal_list a"#;
    pub const GALLERY_IMAGE_SRC: &'static str = r#".gal_list a img"#;
    pub const GALLERY_IMAGE_FULL_SIZE_SRC: &'static str = r#".gal_full a img"#;
    pub const MODEL_VIDEOS: &'static str = r#".video_list a"#;
    // Use # for id's and . for classes.
    pub const MODEL_NOT_FOUND: &'static str = "#cuerpo";
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Girl {
    /// This is the main container for a model from the kindgirls.com website.
    #[serde(rename = "is_single_gallery")]
    pub(crate) is_single_gallery: bool,

    #[serde(rename = "last_update", skip_serializing_if = "Option::is_none")]
    pub(crate) last_update: Option<String>,

    #[serde(rename = "bio")]
    pub(crate) bio: Bio,

    #[serde(rename = "content")]
    pub(crate) content: Visuals,

    #[serde(rename = "stats")]
    pub(crate) stats: Stats,
}

impl Girl {
    pub fn is_single_gallery(url: &str) -> bool {
        !url.contains("girls.php?id=")
    }
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

    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub(crate) link: Option<String>,
}

pub(crate) struct BioName(Option<String>);

impl fmt::Display for BioName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "No name available."),
        }
    }
}

impl Bio {
    pub fn new(info: Vec<String>, url: &str) -> Self {
        let mut bio = Bio {
            name: None,
            country: None,
            birth_year: None,
            alias: None,
            link: Some(url.to_string()),
        };

        for (index, item) in info.iter().enumerate() {
            match index {
                0 => bio.name = Some(item.clone()),
                1 => {
                    if item.starts_with("Alias:") {
                        bio.alias = Some(Self::parse_alias(item))
                    } else {
                        bio.country = Some(item.clone())
                    }
                }
                2 => bio.birth_year = Some(item.clone()),
                3 => bio.alias = Some(Self::parse_alias(item)),
                _ => break,
            }
        }
        bio
    }

    pub fn get_name(&self) -> BioName {
        BioName(self.name.clone())
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

    #[serde(rename = "photos", skip_serializing_if = "Option::is_none")]
    pub(crate) photos: Option<Vec<String>>,

    #[serde(rename = "total_photos", skip_serializing_if = "Option::is_none")]
    pub(crate) total_photos: Option<i32>,
}

#[allow(dead_code)]
pub(crate) struct GalleryLink(Option<String>);

impl fmt::Display for GalleryLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(link) => write!(f, "{}", link),
            None => write!(f, "No gallery link available."),
        }
    }
}

impl Gallery {
    #[allow(dead_code)]
    pub fn show_link(&self) -> GalleryLink {
        GalleryLink(self.link.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Visuals {
    #[serde(rename = "galleries")]
    pub(crate) galleries: Vec<Gallery>,

    #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
    pub(crate) videos: Option<Vec<Video>>,
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Stats {
    #[serde(rename = "total_galleries")]
    pub(crate) total_galleries: usize,

    #[serde(rename = "total_photos")]
    pub(crate) total_photos: i32,

    #[serde(rename = "total_videos", skip_serializing_if = "Option::is_none")]
    pub(crate) total_videos: Option<usize>,
}
