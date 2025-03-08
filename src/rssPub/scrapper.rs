use dom_smoothie::{Article, Config, Readability, ReadabilityError};
use crate::rssPub::epub_data::EpubData;
use scraper::{Html, Selector};
pub fn get_all_content(rss_item:&rss::Item) ->Result<EpubData,&str>{
        println!("partial feed");
        if let Some(link) = rss_item.link() {
            let cons = get_web_page_content(link);
            let con=match cons {
                Ok(asw) => {asw}
                Err(x) => {
                    println!("{}",x);
                    panic!()}
            };
            let titl=rss_item.title().unwrap().to_string();
            let s=match rss_item.content() {
                None => {""}
                Some(s) => {s}
            };
            let epub_data = EpubData {
                title:titl,
                content_text:s.to_string(),
                content:con
            };
           return Ok(epub_data);
        }
        return Err("unable to process");

}

fn get_web_page_content(url:&str) -> Result<Article, &str> {
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let cfg = Config {
        max_elements_to_parse: 9000,
        ..Default::default()
    };
    let asd=Html::parse_document(&response);
    //println!("uat == {} \n data =={} ",&url,&response);
    let mut readabilitys = Readability::new(asd.html(), Some(url), Some(cfg));
    match readabilitys {
        Ok(mut readability) => {
            let articles = readability.parse();
            match articles {
                Ok(article) => {Ok(article)}
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