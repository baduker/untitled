use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author = "baduker",
    version = env!("CARGO_PKG_VERSION"),
    about = "A modest untitled CLI tool to fetch titled images from the web(site",
    long_about = "This is a simple CLI tool to interact, so to speak, with \
    the girls from the kindgirls.com website. It's a work in progress, \
    so don't expect too much from it. It's just an odd way to learn Rust."
)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE", default_value = "untitled.toml")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Configuration options")]
    Config {
        /// Print the configuration file
        #[arg(short, long)]
        print: bool,
    },
    #[command(about = "Get some girls")]
    Scrape {
        /// The URL to scrape from
        #[arg(short, long, value_name = "URL")]
        url: Option<String>,
    },
}
