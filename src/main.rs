use std::collections::HashMap;

use util::downloader::download_image;

mod simple;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple::hash_map::using_hash_map();

    for i in 1..77 {
        for j in 1..7 {
            let file_name = format!("{}-{}.jpg", i, j);
            let image_url = format!(
                "https://www.retio.or.jp/info/ebook/baibai/page{}/x2/{}.jpg?c=202208051751520900",
                i, j
            );

            match download_image(&image_url, &file_name).await {
                Ok(_) => println!("image saved successfully {}:{}", i, j),
                Err(e) => println!("error while downloading image {}:{} : {}", i, j, e),
            }
        }
    }

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}
