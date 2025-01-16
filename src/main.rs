mod cli;
mod config;
mod scraper;
mod utilities;

use crate::scraper::collector::scrape;
use crate::utilities::{format_duration, validate_id};
use clap::Parser;
use cli::{Cli, Commands};
use config::{print_config, read_or_create_config, MyConfig};

const BASE_URL: &str = "https://kindgirls.com/old/girls.php?id=";
fn main() {
    let start_timer = std::time::Instant::now();
    let cli = Cli::parse();
    match read_or_create_config::<MyConfig>() {
        Ok(config) => {
            match cli.command {
                Some(Commands::Config { print }) => {
                    if print {
                        print_config(&config);
                    }
                }
                Some(Commands::Scrape {
                    url,
                    id,
                    list_ids,
                    full_size_image,
                }) => match (url, id, list_ids) {
                    (Some(url), None, None) => scrape(&config, Some(&url), full_size_image),
                    (None, Some(id), None) => {
                        if !validate_id(&id) {
                            eprintln!(
                            "Girl's page ID's are only numbers! Double check the id and try again."
                        );
                            return;
                        }
                        let gallery_url = format!("{}{}", BASE_URL, id);
                        scrape(&config, Some(&gallery_url), full_size_image)
                    }
                    (None, None, Some(list_ids)) => {
                        for id in list_ids {
                            if !validate_id(&id) {
                                eprintln!("Error: ID's are only numbers! Double check the id and try again.");
                                return;
                            }
                            let gallery_url = format!("{}{}", BASE_URL, id);
                            // TODO: It'd be nice to implement multi-threading here
                            scrape(&config, Some(&gallery_url), full_size_image)
                        }
                    }
                    (None, None, None) => {
                        eprintln!("You need to specify either a girl's page URL or ID!")
                    }
                    _ => eprintln!("You can't use both an URL and IDs at the same time!"),
                },
                Some(Commands::Update {
                    auto_approve,
                    parallel,
                    workers,
                }) => {
                    if auto_approve {
                        println!("Running the update in the AUTO-APPROVE mode.");
                    }
                    scraper::updater::Updater::update(&config, auto_approve, parallel, workers)
                        .unwrap();
                }
                None => eprintln!("No command specified! Use --help to see available commands."),
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    let execution_time = start_timer.elapsed();
    format_duration(execution_time);
}
