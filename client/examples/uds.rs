extern crate hyper;

use hyper::Client;
use std::os::unix::net::{UnixListener, UnixStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let unix_stream = UnixStream::connect("/var/wasmship.sock").unwrap();
    let client = Client::builder().build(unix_stream);
    // Parse an `http::Uri`...
    let uri = "http://httpbin.org/ip".parse()?;

    // Await the response...
    let mut resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    Ok(())
}