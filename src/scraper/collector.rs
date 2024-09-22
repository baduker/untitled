use crate::config::Config;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Error;
use scraper::{Html, Selector};

use super::structs::Selectors;

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0";
pub fn scrape<T: Config>(config: &T, url: Option<&str>) {
    match url {
        Some(url) => {
            println!("Scraping from: {}", url);
            let body = fetch(url);
            let document = Html::parse_document(&body.unwrap());
            let selector = Selector::parse(Selectors::MODEL_GALLERIES).unwrap();
            let galleries: Vec<String> = document
                .select(&selector)
                .map(|element| {
                    let href = element.value().attr("href").unwrap();
                    let title = element.value().attr("title").unwrap();
                    format!("{} - {}{}", title, config.base_url(), href)
                })
                .collect();
            for gallery in galleries {
                println!("{}", gallery);
            }
        }
        None => {
            println!("Scraping from: {}", config.base_url());
            println!("Download directory: {}", config.download_dir());
        }
    }
}

fn fetch(url: &str) -> Result<String, Error>{
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
    let response = client.get(url).headers(headers).send();
    let body = response?.text()?;
    Ok(body)
}
