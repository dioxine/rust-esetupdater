// use clap::builder::Str;
use http_body_util::{BodyExt, Full};
use hyper::Uri;
use hyper::body::Bytes;
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
// use hyper_util::client::legacy::connect::HttpConnector;

use super::errors::AppError;

#[tokio::main]
pub async fn read_remote_ini_file(url: &str) -> Result<Vec<u8>, AppError> {
    let url = url.parse::<Uri>()?;

    // Get the host and the port
    let scheme = url.scheme_str().expect("uri has no scheme");
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}://{}:{}", scheme, host, port);
    println!("Address: {}", address);

    // Create the Hyper client
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, Full<Bytes>>(https);

    // Create GET request
    let mut res = client.get(url).await?;
    // let status = res.status();
    let body = res.body_mut().collect().await?;
    let vec = body.to_bytes().to_vec();
    Ok(vec)
}
