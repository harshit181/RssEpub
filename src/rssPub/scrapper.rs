use dom_smoothie::{Article, CandidateSelectMode, Config, Readability, ReadabilityError, TextMode};
use reqwest::blocking::Client;
use crate::rssPub::epub_data::EpubData;
use scraper::{Html, Selector};
pub fn get_all_content(rss_item:&rss::Item) ->Result<EpubData,&str>{
        println!("partial feed");
        if let Some(link) = rss_item.link() {
            let cons = get_web_page_content(link);
            let con:Option<String>=match cons {
                Ok(asw) => {Some(asw)}
                Err(x) => {
                    println!("{} --- {}", x, link);
                    None
                }
            };
            let titl=rss_item.title().unwrap().to_string();
            let content_from_rss =match rss_item.content() {
                None => {""}
                Some(s) => {s}
            };
            match con{
                None => {}
                Some(ads) => {
                    let epub_data = EpubData {
                        title:titl,
                        content_text: content_from_rss.to_string(),
                        content:ads
                    };
                    return Ok(epub_data);
                }
            }

        }
        return Err("unable to process");

}

fn get_web_page_content(url:&str) -> Result<String, &str> {
    let client=Client::builder().use_rustls_tls().build().unwrap();
    let response = client.get(url).send().unwrap().text().unwrap();
    let cfg = Config {
        max_elements_to_parse: 9000,
        text_mode:TextMode::Formatted,
        ..Default::default()
    };
    let asd=Html::parse_document(&response);
    //println!("uat == {} \n data =={} ",&url,&response);
    let mut readabilitys = Readability::new(asd.html(), Some(url), Some(cfg));
    match readabilitys {
        Ok(mut readability) => {
            let articles = readability.parse();
            match articles {
                Ok(article) => {Ok(article.text_content.to_string())}
                Err(sd) => {
                    println!("{}",sd);
                    Err("error while parsing")}
            }

        }
        Err(s) => {
            println!("{} sdsds",s);
            Err("error while parsing")
        }
    }


}