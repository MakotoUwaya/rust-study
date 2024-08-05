use anyhow::Result;
use std::{
    fs::File,
    io::{copy, Cursor},
};

#[allow(dead_code)]
pub async fn download_image(url: &str, file_name: &str) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut content = Cursor::new(response.bytes().await?);
    let mut file = File::create(file_name)?;
    copy(&mut content, &mut file)?;
    Ok(())
}
