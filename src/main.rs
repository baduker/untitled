mod config;
mod cli;

use clap::Parser;
use cli::Cli;
use config::{read_or_create_config, Config, MyConfig};

fn main() {
    let cli = Cli::parse();
    match read_or_create_config::<MyConfig>() {
        Ok(config) => {
            if cli.print {
                print_config(config);
            } else {
                println!("Config loaded successfully!");
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn print_config<T: std::fmt::Debug + Config>(config: T) {
    println!("Config: {:?}", config);
    println!("base_url: {:?}", config.base_url());
}
