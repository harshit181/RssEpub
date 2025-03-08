use std::str::FromStr;
use opml::{Error, OPML};
use crate::rssPub::scrapper::get_all_content;


fn get_urls(opml: &str) -> Vec<String> {
        let mut urls: Vec<String> = Vec::new();
        let document = OPML::from_str(opml);
        match document {
            Ok(x) => {
                let body = x.body;
                let outlines = body.outlines;
                for outline in outlines {
                    let xml_url = outline.xml_url;
                    match xml_url {
                        Some(url) => {
                            urls.push(url);
                        }
                        None => {}
                    }
                }
            }
            Err(_) => {}
        }
        urls
    }


pub fn get_content(url: &str, days:i64) -> Vec<rss::Item>{
    let response = reqwest::blocking::get(url).unwrap();
    let content = response.text().unwrap();
    let channel = rss::Channel::from_str(&content).unwrap();
    let items = channel.items();
    let mut selected_items: Vec<rss::Item> = Vec::new();
    let one_day_ago = chrono::Utc::now() - chrono::Duration::days(days);
    for item in items {
        if let Some(pub_date) = item.pub_date() {
            if let Ok(date) = chrono::DateTime::parse_from_rfc2822(pub_date) {
                if date > one_day_ago {
                    let s=item.clone();
                    selected_items.push(s);
                }
            }
        }
    }
    selected_items
}



