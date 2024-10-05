use crate::config::Config;
use crate::scraper::structs::Girl;
use crate::utilities::{format_date, to_snake_case};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::error::Error;
use std::fs;
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
        let base_dir = get_base_dir(config)?;

        create_base_dirs(config, girl)?;
        print_gallery_structure(&base_dir, girl)?;
        print_video_structure(&base_dir, girl)?;

        download_galleries(&base_dir, girl)?;
        download_videos(&base_dir, girl)?;

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

fn download_galleries(base_dir: &PathBuf, girl: &Girl) -> Result<(), Box<dyn Error>> {
    let girl_name = to_snake_case(&girl.bio.get_name().to_string());
    let client = Client::new();
    let total_galleries = girl.content.galleries.len();
    let mut current_gallery = 0;

    for gallery in &girl.content.galleries {
        if let (Some(date), Some(photos)) = (&gallery.date, &gallery.photos) {
            let formatted_date = format_date(date).unwrap_or_else(|| "unknown_date".to_string());
            let gallery_dir = base_dir
                .join(&girl_name)
                .join("galleries")
                .join(&formatted_date);

            fs::create_dir_all(&gallery_dir)?;

            current_gallery += 1;
            println!(
                "{}'s gallery {} of {} ({})",
                girl.bio.get_name(),
                current_gallery,
                total_galleries,
                formatted_date
            );

            let progress_bar = ProgressBar::new(photos.len() as u64);
            progress_bar.set_message(format!("Downloading gallery {}", formatted_date));
            progress_bar.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap());

            for (index, photo_url) in photos.iter().enumerate() {
                let response = client.get(photo_url).send()?;
                let file_name = format!("{:03}.jpg", index + 1);
                let file_path = gallery_dir.join(file_name);

                let mut file = fs::File::create(file_path)?;
                let content = response.bytes()?;
                std::io::copy(&mut content.as_ref(), &mut file)?;

                progress_bar.inc(1);
            }

            progress_bar.finish_with_message("All photos downloaded!");
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

fn download_videos(base_dir: &PathBuf, girl: &Girl) -> Result<(), Box<dyn Error>> {
    if let Some(videos) = &girl.content.videos {
        let girl_name = to_snake_case(&girl.bio.get_name().to_string());
        let client = Client::new();
        let total_videos = videos.len();

        for (video_index, video) in videos.iter().enumerate() {
            if let (Some(link), Some(source)) = (&video.link, &video.source) {
                let video_dir = base_dir.join(&girl_name).join("videos");
                fs::create_dir_all(&video_dir)?;

                let file_name = format!("video_{:03}.mp4", video_index + 1);
                let file_path = video_dir.join(&file_name);

                println!(
                    "Downloading video {} of {} ({})",
                    video_index + 1,
                    total_videos,
                    link
                );

                let mut response = client.get(source).send()?;
                let total_size = response.content_length().unwrap_or(0);

                let progress_bar = ProgressBar::new(total_size);
                progress_bar.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) - Video {msg}")
                    .unwrap());
                progress_bar.set_message(format!("{}/{}", video_index + 1, total_videos));

                let mut file = fs::File::create(&file_path)?;

                let mut progress_wrapper = progress_bar.wrap_write(&mut file);
                copy(&mut response, &mut progress_wrapper)?;

                progress_bar.finish_with_message(format!(
                    "{}/{} complete",
                    video_index + 1,
                    total_videos
                ));
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
