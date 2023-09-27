use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use reqwest;
use std::io::prelude::*;
use flate2::read::GzDecoder;

#[tokio::main]
async fn main() {
    let landsat_remote_file: String = "https://storage.googleapis.com/gcp-public-data-landsat/index.csv.gz".to_owned();
    let local_gz_file: String = download_file(&landsat_remote_file).await.unwrap();
    println!("Downloaded file: {}", local_gz_file);
    let local_file = unpack_gz(&local_gz_file).unwrap();
    let raw_string = read_file(&local_file).unwrap();
}

async fn download_file(url: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(url).await?.bytes().await?;
    let mut out = File::create("index.csv.gz")?;
    out.write_all(&resp[..])?;
    Ok("index.csv.gz".to_owned())
}

fn read_file(file: &str) -> Result<String, Box<dyn Error>> {
    let mut raw_file = File::open(file).unwrap();
    let mut contents = String::new();
    raw_file.read_to_string(&mut contents).unwrap();
    Ok(contents)
}

fn unpack_gz(path: &str) -> Result<String, Box<dyn Error>> {
    let local_gz = File::open(path)?; 
    let mut tar = GzDecoder::new(local_gz);
    let mut s = String::new();
    tar.read_to_string(&mut s)?;
    let mut file = File::create("index.csv")?;
    file.write_all(s.as_bytes())?;
    Ok("index.csv".to_owned())
}

