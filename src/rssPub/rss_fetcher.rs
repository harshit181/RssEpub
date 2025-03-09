use std::str::FromStr;
use opml::{Error, OPML};
use rss::Channel;

pub fn get_urls(opml: &str) -> Vec<(String, String)> {

        let mut urls: Vec<(String,String)> = Vec::new();
        let document = OPML::from_str(opml);
        match document {
            Ok(x) => {
                let body = x.body;
                let outlinesx = body.outlines;
                for outlines in outlinesx {
                    for outline in outlines.outlines {
                    let xml_url = outline.xml_url;
                    match xml_url {
                        Some(url) => {
                            urls.push((outline.title.unwrap(), url));
                        }
                        None => { println!("ddddd") }
                    }
                }
            }
        }
            Err(_) => {println!("xxxxx")}
        }
        urls
    }


pub fn get_content(url: &str, days:i64) -> Vec<rss::Item>{
    let responses = reqwest::blocking::get(url);
    let mut selected_items: Vec<rss::Item> = Vec::new();
    match responses {
        Ok(response) => {
            let content = response.text().unwrap();
            let channels = rss::Channel::from_str(&content);
            match channels{
                Ok(channel) => {
                    let items = channel.items();
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
                }
                Err(aa) => {
                    println!("Error data {}",aa);
                }
            }
        }
        Err(error) => {println!("Error while fetching {}",error);
        }
    }


    selected_items
}



