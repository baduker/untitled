use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const CONFIG_FILE: &str = ".untitled.toml";

pub trait Config: Default {
    fn app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    fn base_url(&self) -> &str;
    fn download_dir(&self) -> &str;
    fn is_active(&self) -> bool;
}

impl Config for MyConfig {
    fn app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn download_dir(&self) -> &str {
        &self.download_dir
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MyConfig {
    pub app_version: String,
    pub base_url: String,
    pub download_dir: String,
    pub is_active: bool,
}

impl Default for MyConfig {
    fn default() -> Self {
        MyConfig {
            app_version: MyConfig::app_version().to_string(),
            base_url: "https://kindgirls.com/old".to_string(),
            download_dir: "kindgirls".to_string(),
            is_active: true,
        }
    }
}

pub fn read_or_create_config<T: DeserializeOwned + Serialize + Config>(
) -> Result<T, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        create_default_config::<T>(&config_path)?;
    }
    let config_str = fs::read_to_string(config_path)?;
    let config: T = toml::from_str(&config_str)?;
    Ok(config)
}

fn create_default_config<T: Serialize + Config>(
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let default_config = T::default();
    let toml_string = toml::to_string(&default_config)?;
    let mut file = fs::File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    println!("Created default config at {}", path.display());
    Ok(())
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = dirs::home_dir().ok_or("Could not find home directory")?;
    path.push(CONFIG_FILE);
    Ok(path)
}
