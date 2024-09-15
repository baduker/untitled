mod config;

use config::{read_or_create_config, Config, MyConfig};

fn main() {
    match read_or_create_config::<MyConfig>() {
        Ok(config) => print_config(config),
        Err(e) => eprintln!("Error reading or creating config: {}", e),
    }
}

fn print_config<T: std::fmt::Debug + Config>(config: T) {
    println!("Config: {:?}", config);
    println!("base_url: {:?}", config.base_url());
}
