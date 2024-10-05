mod cli;
mod config;
mod scraper;
mod utilities;

use crate::scraper::collector::scrape;
use clap::Parser;
use cli::{Cli, Commands};
use config::{print_config, read_or_create_config, MyConfig};

fn main() {
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
                full_size_image,
            }) => match url {
                Some(url) => scrape(&config, Some(&url), full_size_image),
                None => scrape(&config, None, false),
            },
            None => eprintln!("No command specified! Use --help to see available commands."),
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
