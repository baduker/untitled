use crate::config::Config;
use crate::scraper::structs::Girl;
use crate::utilities::{format_date, to_snake_case};
use reqwest::blocking::Client;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

pub trait Downloader {
    fn download<T: Config>(
        &self,
        config: &T,
        girl: &Girl,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct DownloaderImpl;

impl Downloader for DownloaderImpl {
    fn download<T: Config>(&self, config: &T, girl: &Girl) -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let base_dir = config.download_dir();

        // Create directory for the girl
        let girl_dir = format!("{}/{}", base_dir, girl.bio.get_name());
        create_dirs(config, girl)?;

        // Download galleries
        for gallery in &girl.content.galleries {
            if let Some(link) = &gallery.link {
                let gallery_dir = format!(
                    "{}/{}",
                    girl_dir,
                    gallery.date.as_deref().unwrap_or("unknown_date")
                );
                create_dirs(config, girl)?;
                download_file(&client, link, &gallery_dir)?;
            }
        }

        // Download videos
        if let Some(videos) = &girl.content.videos {
            for video in videos {
                if let Some(source) = &video.source {
                    let video_dir = format!("{}/videos", girl_dir);
                    create_dirs(config, girl)?;
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
    let content = response.bytes()?;
    copy(&mut content.as_ref(), &mut dest)?;
    Ok(())
}

pub fn create_dirs<T: Config>(config: &T, girl: &Girl) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Impossible to get your home dir")?;
    let base_dir = PathBuf::from(home_dir.join(config.download_dir()));
    let girl_name = to_snake_case(&girl.bio.get_name().to_string());

    let mut paths_to_create = vec![
        base_dir.join(&girl_name),
        base_dir.join(&girl_name).join("galleries"),
        base_dir.join(&girl_name).join("videos"),
    ];

    // Add gallery paths
    for gallery in &girl.content.galleries {
        if let Some(date) = &gallery.date {
            let formatted_date = format_date(date).unwrap_or_else(|| "unknown_date".to_string());
            paths_to_create.push(base_dir.join(&girl_name).join(formatted_date));
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
