use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::scraper::collector::{collect_girl, fetch};
use crate::scraper::downloader::Downloader;
use crate::scraper::structs::Girl;
use crate::utilities::today_date;

pub(crate) struct Updater;

impl Updater {
    pub fn update<T: Config>(config: &T) -> Result<(), Box<dyn std::error::Error>> {
        let base_dir = Self::get_base_dir(config)?;
        if !base_dir.exists() {
            return Err("Base directory does not exist! Nothing to do.".into());
        }

        let directories = fs::read_dir(base_dir)?;
        for dir in directories {
            let dir = dir?;
            let path = dir.path();
            if path.is_dir() {
                Self::update_girl_content(config, &dir.path())?;
            }
        }
        Ok(())
    }

    fn update_girl_content<T: Config>(
        config: &T,
        girl_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json_files: Vec<_> = fs::read_dir(girl_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "json"))
            .collect();

        for json_file in json_files {
            let content = fs::read_to_string(json_file.path())?;
            let mut existing_girl: Girl = serde_json::from_str(&content)?;

            if let Some(link) = &existing_girl.bio.link {
                println!(
                    "Checking for new content for {}",
                    existing_girl.bio.get_name()
                );

                let body = fetch(link)?;
                let document = scraper::Html::parse_document(&body);
                let new_girl = collect_girl(link, &document, true);

                let has_new_content = Self::compare_content(&existing_girl, &new_girl);

                if has_new_content {
                    println!("New content found!");
                    Self::prompt_and_download(config, &new_girl)?;

                    // Update the existing girl with new content and timestamp
                    existing_girl.is_single_gallery = false;
                    existing_girl.content = new_girl.content;
                    existing_girl.stats = new_girl.stats;
                    existing_girl.last_update = Some(today_date());

                    // Save updated JSON
                    let json = serde_json::to_string_pretty(&existing_girl)?;
                    fs::write(json_file.path(), json)?;
                }
            }
        }

        Ok(())
    }

    fn compare_content(existing: &Girl, new: &Girl) -> bool {
        let existing_galleries = existing.content.galleries.len();
        let new_galleries = new.content.galleries.len();

        let existing_videos = existing
            .content
            .videos
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);

        let new_videos = new.content.videos.as_ref().map(|v| v.len()).unwrap_or(0);

        // This is a simple quantity comparison, which should be enough for now
        new_galleries > existing_galleries || new_videos > existing_videos
    }

    fn prompt_and_download<T: Config>(
        config: &T,
        girl: &Girl,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Would you like to download the new content? (y/n)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            let downloader = crate::scraper::downloader::DownloaderImpl;
            downloader.download(config, girl)?;
        }

        Ok(())
    }

    fn get_base_dir<T: Config>(config: &T) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir().ok_or("Impossible to get your home dir")?;
        Ok(Path::join(&home_dir, config.download_dir()))
    }
}
