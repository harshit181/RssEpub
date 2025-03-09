use std::collections::HashMap;
use crate::rssPub::db_loader::load_rss_feeds;
use crate::rssPub::epub_data::EpubData;
use crate::rssPub::rss_fetcher::get_content;
use crate::rssPub::scrapper::get_all_content;
use crate::rssPub::epub_generator::generate_epub;
pub fn load_data(){
    let mut data:HashMap<String,Vec<EpubData>>=HashMap::new();
    let feed_from_db=load_rss_feeds();
    for feed in feed_from_db {
        let rss_items=get_content(&feed.rss_link,1);
        let mut rss_epub_data_vec:Vec<EpubData>=Vec::new();
        for rss_item in rss_items {
            let rss_epub_datas = get_all_content(&rss_item);
            match rss_epub_datas{
                Ok(rss_epub_data) => { rss_epub_data_vec.push(rss_epub_data)}
                Err(_) => {println!("not processing {:?}",&rss_item.title)}
            }
        }
        data.insert(feed.name,rss_epub_data_vec);
    }

    let sss=generate_epub(data);
    match sss {
        Ok(a)=>{},
        Err(s)=>{println!("{}",s)}
    }
}
