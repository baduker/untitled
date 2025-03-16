use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::scraper::collector::{collect_girl, fetch};
use crate::scraper::downloader::Downloader;
use crate::scraper::structs::Girl;
use crate::utilities::{get_base_dir, today_date};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

pub(crate) struct Updater;

impl Updater {
    /// Updates the content based on the given configuration.
    ///
    /// * `config`: A reference to the configuration (must be `Send + Sync`).
    /// * `auto_approve`: If `true`, all updates proceed without user prompting.
    /// * `parallel`: If `true`, updates run in multiple threads (when `workers > 1`).
    /// * `workers`: Number of threads to use if `parallel` is enabled (default: 4).
    pub fn update<T: Config + Send + Sync>(
        config: &T,
        auto_approve: bool,
        parallel: bool,
        workers: usize,
    ) -> Result<()> {
        let base_dir = get_base_dir(config)?;
        if !base_dir.exists() {
            return Err(anyhow::anyhow!("Directory does not exist: {:?}", base_dir));
        }

        let directories: Vec<_> = fs::read_dir(base_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();

        if parallel && workers > 1 {
            // Build a custom Rayon thread pool with the specified number of workers.
            let pool = ThreadPoolBuilder::new().num_threads(workers).build()?;

            // If auto_approve is true, we can parallelize easily. If not, prompting
            // for user input in multiple threads may lead to confusing output.
            // In this example, we still parallelize even if auto_approve = false,
            // but be aware that this could cause multiple prompts from different threads.
            // A common approach is to only parallelize when auto_approve is true.
            pool.install(|| {
                directories.par_iter().try_for_each(|dir| {
                    Self::update_girl_content(config, dir, auto_approve, parallel)
                })
            })?;
        } else {
            // Sequential update (no parallelism)
            for dir in directories {
                Self::update_girl_content(config, &dir, auto_approve, parallel)?;
            }
        }

        Ok(())
    }

    fn update_girl_content<T: Config + Send + Sync>(
        config: &T,
        girl_dir: &Path,
        auto_approve: bool,
        parallel: bool,
    ) -> Result<()> {
        let json_files: Vec<PathBuf> = fs::read_dir(girl_dir)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().map_or(false, |ext| ext == "json"))
            .collect();

        if parallel {
            json_files.par_iter().try_for_each(|json_path| {
                Self::process_girl_file(config, json_path, auto_approve, true)
            })?;
        } else {
            for json_path in json_files {
                Self::process_girl_file(config, &json_path, auto_approve, false)?;
            }
        }

        Ok(())
    }

    fn process_girl_file<T: Config>(
        config: &T,
        json_path: &Path,
        auto_approve: bool,
        parallel_run: bool,
    ) -> Result<()> {
        let content = fs::read_to_string(json_path)?;
        let mut existing_girl: Girl = serde_json::from_str(&content)?;

        if let Some(link) = &existing_girl.bio.link {
            println!(
                "Checking {}'s page for new content...",
                existing_girl.bio.get_name()
            );

            let body = fetch(link)?;
            let document = scraper::Html::parse_document(&body);
            let new_girl = collect_girl(link, &document, true);

            let has_new_content = Self::compare_content(&existing_girl, &new_girl);

            if has_new_content {
                println!("New content found!");
                if auto_approve || Self::prompt_user()? {
                    Self::download(config, &new_girl, auto_approve, parallel_run)?;

                    // Update the existing girl
                    existing_girl.is_single_gallery = false;
                    existing_girl.content = new_girl.content;
                    existing_girl.stats = new_girl.stats;
                    existing_girl.last_update = Some(today_date());

                    let json = serde_json::to_string_pretty(&existing_girl)?;
                    fs::write(json_path, json)?;
                } else {
                    println!("Update skipped for {}", existing_girl.bio.get_name());
                }
            }
        }

        Ok(())
    }

    fn download<T: Config>(
        config: &T,
        new_girl: &Girl,
        auto_approve: bool,
        parallel_run: bool,
    ) -> Result<()> {
        let downloader = crate::scraper::downloader::DownloaderImpl;
        downloader
            .download(config, new_girl, auto_approve, parallel_run)
            .expect("TODO: panic message");
        Ok(())
    }

    fn compare_content(existing: &Girl, new: &Girl) -> bool {
        let existing_galleries = existing.content.galleries.len();
        let new_galleries = new.content.galleries.len();

        let existing_videos = existing.content.videos.as_ref().map_or(0, |v| v.len());
        let new_videos = new.content.videos.as_ref().map_or(0, |v| v.len());

        // Simple quantity comparison
        new_galleries > existing_galleries || new_videos > existing_videos
    }

    fn prompt_user() -> Result<bool> {
        println!("Do you want to approve this update? (y/n):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().eq_ignore_ascii_case("y"))
    }
}
