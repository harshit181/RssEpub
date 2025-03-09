use std::collections::HashMap;
use std::fs::File;
use std::ops::Add;
use epub_builder::{Toc, TocElement};
use crate::rssPub::epub_data::EpubData;


pub fn generate_epub(data:HashMap<String,Vec<EpubData>>) ->epub_builder::Result<File>{

    let mut basic_check:HashMap<String,i32>= HashMap::new();
    let mut output_file = File::create("NewsFeed.epub")?;
    let mut builder = epub_builder::EpubBuilder::new(epub_builder::ZipLibrary::new()?)?;
        builder.metadata("author", "RssEpub")?
            .metadata("title", "NewsFeed")?
            .epub_version(epub_builder::EpubVersion::V30)
            //.add_cover_image("cover.png", dummy_image.as_bytes(), "image/png")?
            .inline_toc();
            builder.inline_toc();
        for (key,value) in data{
            let articles =key.as_str();
            let page_name=key.clone()+".xhtml";
            let sd=Toc::new().add(TocElement::new(page_name, articles));
            for item in value{
                let mut title=item.title.to_string()+".xhtml";
                let temp_key=item.title.to_string();
                if basic_check.contains_key(&temp_key)
                {
                    title=title+basic_check.get(&temp_key).unwrap().to_string().as_str();
                   let s= basic_check.get(&temp_key).unwrap().add(1);
                    basic_check.insert(temp_key,s);
                }
                else{
                    basic_check.insert(temp_key,1);
                }
                println!("adding {}",&title);
                println!("***{}",&item.content_text);
                let text=item.content.to_string();
                println!("---{}",&text);
                builder.add_content(epub_builder::EpubContent::new(format!("{}.xhtml",title), text.as_bytes()).title(item.title))?;
            }
        }
    builder.generate(&mut output_file)?;
    Ok(output_file)


}