use std::error::Error;
use std::fs;
use std::path::Path;

use crate::config::Config;

pub fn splitter(string: &str, split_on: &str) -> Vec<String> {
    string.split(split_on).map(|s| s.to_string()).collect()
}

pub fn create_dir<T: Config>(config: &T, dir_name: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(dir_name).exists() {
        fs::create_dir(dir_name)?;
    }
    Ok(())
}
