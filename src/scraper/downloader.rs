use crate::config::Config;
use crate::scraper::structs::Girl;
use crate::utilities::{format_date, to_snake_case};
use std::error::Error;
use std::fs;
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
        let base_dir = get_base_dir(config)?;

        create_base_dirs(config, girl)?;
        print_gallery_structure(&base_dir, girl)?;
        print_video_structure(&base_dir, girl)?;

        Ok(())
    }
}

fn print_gallery_structure(base_dir: &PathBuf, girl: &Girl) -> Result<(), Box<dyn Error>> {
    let girl_name = to_snake_case(&girl.bio.get_name().to_string());

    for gallery in &girl.content.galleries {
        if let Some(date) = &gallery.date {
            let formatted_date = format_date(date).unwrap_or_else(|| "unknown_date".to_string());
            let gallery_dir = base_dir
                .join(&girl_name)
                .join("galleries")
                .join(&formatted_date);
            println!("Gallery directory: {:?}", gallery_dir);
            println!("Gallery link: {}", gallery.show_link());
        }
    }

    Ok(())
}

fn print_video_structure(base_dir: &PathBuf, girl: &Girl) -> Result<(), Box<dyn Error>> {
    if let Some(videos) = &girl.content.videos {
        let girl_name = to_snake_case(&girl.bio.get_name().to_string());

        for (index, video) in videos.iter().enumerate() {
            if let Some(link) = &video.link {
                let video_file = base_dir
                    .join(&girl_name)
                    .join("videos")
                    .join(format!("video_{}.mp4", index + 1));
                println!("Video file: {:?} (from {})", video_file, link);
            }
        }
    }

    Ok(())
}

fn get_base_dir<T: Config>(config: &T) -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Impossible to get your home dir")?;
    Ok(PathBuf::from(home_dir.join(config.download_dir())))
}

pub fn create_base_dirs<T: Config>(config: &T, girl: &Girl) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Impossible to get your home dir")?;
    let base_dir = PathBuf::from(home_dir.join(config.download_dir()));
    let girl_name = to_snake_case(&girl.bio.get_name().to_string());

    let mut paths_to_create = vec![
        base_dir.join(&girl_name),
        base_dir.join(&girl_name).join("galleries"),
    ];

    // Create the videos dir only if the struct has any

    if girl.content.videos.is_some() {
        paths_to_create.push(base_dir.join(&girl_name).join("videos"));
    }

    // Create all the base directories
    for path in paths_to_create {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
    }

    Ok(())
}
