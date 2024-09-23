use super::structs::{Gallery, Selectors};
use crate::config::Config;
use crate::utilities::splitter;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Error;
use scraper::{Html, Selector};

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0";
const DEFAULT_BASE_URL: &str = "https://kindgirls.com";

pub fn scrape<T: Config>(config: &T, url: Option<&str>) {
    match url {
        Some(url) => {
            println!("Scraping from: {}", url);
            let body = fetch(url);
            let document = Html::parse_document(&body.unwrap());
            let galleries = collect_gallery(&document);

            let total_images: i32 = galleries.iter().filter_map(|g| g.total_photos).sum();

            for gallery in &galleries {
                println!("{:?}", gallery);
            }
            println!("Total images: {}", total_images);
        }
        None => {
            println!("Scraping from: {}", config.base_url());
            println!("Download directory: {}", config.download_dir());
        }
    }
}

fn fetch(url: &str) -> Result<String, Error> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
    let response = client.get(url).headers(headers).send();
    let body = response?.text()?;
    Ok(body)
}

fn collect_gallery(document: &Html) -> Vec<Gallery> {
    let selector = Selector::parse(Selectors::MODEL_GALLERIES).unwrap();
    document
        .select(&selector)
        .map(|element| {
            let href = element.value().attr("href").unwrap();
            let full_url = format!("{}{}", DEFAULT_BASE_URL, href);
            let gallery_id = splitter(href, "=").last().unwrap().to_string();

            let text = element.text().collect::<Vec<_>>().join(" ");

            let total_photos = text.split_whitespace().next().unwrap().parse::<i32>().ok();

            let title = splitter(element.value().attr("title").unwrap(), ", ");
            let date = title.last().unwrap().to_string();

            Gallery {
                id: Some(gallery_id),
                date: Some(date),
                link: Some(full_url),
                total_photos,
            }
        })
        .collect()
}
