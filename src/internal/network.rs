use super::errors::AppError;
use crate::USER_AGENT;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use std::io::copy;

pub fn download_file(url: &str, local_path: &str) -> Result<(), AppError> {
    // Create client and request
    let client = Client::builder().user_agent(USER_AGENT).build()?;

    let mut response = client
        .get(url)
        .basic_auth("", Some(""))
        .send()?
        .error_for_status()?;

    // Create parent directories if needed
    if let Some(parent) = std::path::Path::new(local_path).parent() {
        // println!("Creating parent directory: {:?}", url);
        std::fs::create_dir_all(parent)?;
    }

    // Stream directly to file
    let file = File::create(local_path)?;
    let mut dest_file = BufWriter::new(file);
    copy(&mut response, &mut dest_file)?;

    Ok(())
}

pub fn read_remote_file(url: &str) -> Result<Vec<u8>, AppError> {
    let client = Client::builder().user_agent(USER_AGENT).build()?;

    let mut response = client
        .get(url)
        .basic_auth("", Some(""))
        .send()?
        .error_for_status()?;

    let mut content = Vec::new();
    response.read_to_end(&mut content)?;

    Ok(content)
}
