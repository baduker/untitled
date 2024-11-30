mod cli;
mod config;
mod scraper;
mod utilities;

use crate::scraper::collector::scrape;
use crate::utilities::{format_duration, validate_id};
use clap::Parser;
use cli::{Cli, Commands};
use config::{print_config, read_or_create_config, MyConfig};

fn main() {
    let start_timer = std::time::Instant::now();
    let cli = Cli::parse();
    match read_or_create_config::<MyConfig>() {
        Ok(config) => match cli.command {
            Some(Commands::Config { print }) => {
                if print {
                    print_config(&config);
                }
            }
            Some(Commands::Scrape {
                url,
                id,
                full_size_image,
            }) => match (url, id) {
                (Some(url), None) => scrape(&config, Some(&url), full_size_image),
                (None, Some(id)) => {
                    if !validate_id(&id) {
                        eprintln!(
                            "Girl's page ID's are only numbers! Double check the id and try again."
                        );
                        return;
                    }
                    let constructed_url =
                        format!("https://www.kindgirls.com/old/girls.php?id={}", id);
                    scrape(&config, Some(&constructed_url), full_size_image)
                }
                (None, None) => eprintln!("You need to specify either a girl's page URL or ID!"),
                (Some(_), Some(_)) => eprintln!("You can't use both URL and ID at the same time!"),
            },
            Some(Commands::Update) => {
                scraper::updater::Updater::update(&config).unwrap();
            }
            None => eprintln!("No command specified! Use --help to see available commands."),
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    let execution_time = start_timer.elapsed();
    format_duration(execution_time);
}
