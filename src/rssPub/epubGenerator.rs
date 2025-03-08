use std::collections::HashMap;
use epub_builder::{Toc, TocElement};
use crate::rssPub::epub_data::EpubData;

fn generate_epub(data:HashMap<String,Vec<EpubData>>) ->epub_builder::Result<Vec<u8>>{

    let dummy_image = "Not really a PNG image";
    let mut output = Vec::<u8>::new();

    let mut builder = epub_builder::EpubBuilder::new(epub_builder::ZipLibrary::new()?)?;
        builder.metadata("author", "Joan Doe")?
            .metadata("title", "Dummy Book")?
            .epub_version(epub_builder::EpubVersion::V30)
            .add_cover_image("cover.png", dummy_image.as_bytes(), "image/png")?.inline_toc();
            builder.inline_toc();
        for (key,value) in data{
            let articles =key.as_str();
            let sd=Toc::new().
                add(TocElement::new(".xhtml", articles));
            for item in value{
                //builder.add_content(epub_builder::EpubContent::new(format!("{}.xhtml",item.title), item.content.content.as_bytes()).title(item.title))?;
            }


        }



    builder.generate(&mut output)?;
    Ok(output)





}