// use std::fs::File;

use base64::{Engine as _, engine::general_purpose};
use http_body_util::{BodyExt, Empty};

use hyper::{
    Method, Request, Response, Uri,
    body::{Bytes, Incoming},
    header::{HeaderValue, USER_AGENT},
};

use hyper_tls::HttpsConnector;

use hyper_util::{client::legacy::Client, rt::TokioExecutor};

use futures::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use super::errors::AppError;

async fn make_a_request(
    username: &str,
    password: &str,
    url: &str,
    user_agent: &str,
) -> Result<Response<Incoming>, AppError> {
    let url = url.parse::<Uri>().unwrap();

    // Get the host and the port
    // let scheme = url.scheme_str().expect("uri has no scheme");
    // let host = url.host().expect("uri has no host");
    // let port = url.port_u16().unwrap_or(80);

    // println!("Username is: {}, Password is : {}", username, password);
    // println!("Scheme is {}", scheme);
    // println!("Host is {}", host);
    // println!("Port is {}", port);
    // println!("User-Agent is {}", user_agent);

    // let address = format!("{}://{}:{}", scheme, host, port);
    // println!("Address is: {}", address);

    // Encode the credentials
    let credentials = format!("{}:{}", username, password);
    let encoded_credentials = general_purpose::STANDARD.encode(credentials.as_bytes());

    // Construct the Authorization header
    let auth_header_value = format!("Basic {}", encoded_credentials);

    // Build the request
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header(USER_AGENT, HeaderValue::from_str(user_agent).unwrap_or(HeaderValue::from_static("EEA Update (Windows; U; 64bit; BPC 11.0.2044.0; OS: 10.0.26100 SP 0.0 NT; HWF: 921b979f-686d-4fa2-bebb-3ffe2ab877da; PLOC ru_ru; PCODE 107.0.0; PAR -1; ATH -1; DC 0; PLID 3AC-9SP-9D9; SEAT 154b3474; RET 2107)")))
        .header("Authorization", auth_header_value)
        .body(Empty::<Bytes>::new())
        .unwrap(); // Use Empty for no body

    // println!("Request is: {:?}", req);

    // Create the Hyper client
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);

    // Create GET request and get a response
    let response = client.request(req).await?;
    Ok(response)
}

pub async fn read_remote_ini_file(
    username: &str,
    password: &str,
    url: &str,
    user_agent: &str,
) -> Result<Vec<u8>, AppError> {
    log::info!("⌛️Reading INI: {}", url);
    // let status = res.status();
    let mut res = make_a_request(username, password, url, user_agent).await?;
    let body = res.body_mut().collect().await?;
    let vec = body.to_bytes().to_vec();
    Ok(vec)
}

pub async fn download_file(
    username: &str,
    password: &str,
    url: &str,
    user_agent: &str,
    local_path: &str,
) -> Result<(), AppError> {
    log::info!("⌛️Downloading File: {}", url);

    let res = make_a_request(username, password, url, user_agent).await?;

    // Create parent directories if needed
    if let Some(parent) = std::path::Path::new(local_path).parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Create the output file asynchronously
    let mut file = File::create(local_path).await?;

    // Get the incoming body from the response
    let body = res.into_body();
    let mut stream = body.into_data_stream();

    while let Some(frame) = stream.next().await {
        let frame = frame?;
        file.write_all(&frame).await?;
    }

    file.flush().await?;

    Ok(())
}
