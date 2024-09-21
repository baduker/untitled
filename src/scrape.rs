use crate::config::Config;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Error;

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0";
pub fn scrape<T: Config>(config: &T, url: Option<&str>) {
    match url {
        Some(url) => {
            println!("Scraping from: {}", url);
            fetch(url);
        }
        None => {
            println!("Scraping from: {}", config.base_url());
            println!("Download directory: {}", config.download_dir());
        }
    }
}

fn fetch(url: &str) {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
    let response = client.get(url).headers(headers).send();
    match response {
        Ok(response) => {
            println!("Response: {:?}", response.status());
        }
        Err(e) => {
            eprintln!("Error: {}", Error::to_string(&e));
        }
    }
}
