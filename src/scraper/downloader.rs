use crate::config::Config;
use crate::scraper::structs::{Gallery, Girl, Video, Visuals};
use crate::utilities::create_dir;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub trait Downloader {
    fn download<T: Config>(
        &self,
        config: &T,
        girl: &Girl,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct DownloaderImpl;

impl Downloader for DownloaderImpl {
    fn download<T: Config>(
        &self,
        config: &T,
        girl: &Girl,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let base_dir = config.download_dir();

        // Create directory for the girl
        let girl_dir = format!("{}/{}", base_dir, girl.bio.get_name());
        create_dir(config, &girl_dir)?;

        // Download galleries
        for gallery in &girl.content.galleries {
            if let Some(link) = &gallery.link {
                let gallery_dir = format!(
                    "{}/{}",
                    girl_dir,
                    gallery.date.as_deref().unwrap_or("unknown_date")
                );
                create_dir(config, &gallery_dir)?;
                download_file(&client, link, &gallery_dir)?;
            }
        }

        // Download videos
        if let Some(videos) = &girl.content.videos {
            for video in videos {
                if let Some(source) = &video.source {
                    let video_dir = format!("{}/videos", girl_dir);
                    create_dir(config, &video_dir)?;
                    download_file(&client, source, &video_dir)?;
                }
            }
        }

        Ok(())
    }
}

fn download_file(client: &Client, url: &str, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).send()?;
    let file_name = url.split('/').last().unwrap();
    let file_path = format!("{}/{}", dir, file_name);
    let mut dest = File::create(file_path)?;
    let mut content = response.bytes()?;
    copy(&mut content.as_ref(), &mut dest)?;
    Ok(())
}
