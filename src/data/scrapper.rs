use dioxus::html::li::content;
use dom_smoothie::{Article, Config, Readability, ReadabilityError};
use crate::data::epub_data::EpubData;

fn get_all_content(rss_item:&rss::Item) ->Result<EpubData,Err>{
        println!("partial feed");
        if let Some(link) = rss_item.link() {
            let con = get_web_page_content(link).unwrap();
            let titl=rss_item.title().unwrap().to_string();
            let epub_data = EpubData {
                title:titl,
                content_text:rss_item.content().content(),
                content:con
            };
           return Ok(epub_data);
        }
        return Err("unable to process");

}

fn get_web_page_content(url:&str) -> Result<Article,Err> {
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let cfg = Config {
        max_elements_to_parse: 9000,
        ..Default::default()
    };
    let mut readabilitys = Readability::new(response, Some(url), Some(cfg));

    match readabilitys {
        Ok(mut readability) => {
            let articles = readability.parse();
            return articles
        }
        Err(s) => {
            Err("error while parsing")
        }
    }


}