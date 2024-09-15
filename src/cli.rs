use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE", default_value = "untitled.toml")]
    pub config: PathBuf,

    /// Print the configuration
    #[arg(short, long)]
    pub print: bool,
}
