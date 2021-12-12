use async_trait::async_trait;
use hyper::body::HttpBody;
use hyperlocal::{UnixClientExt, Uri};
use std::error::Error;
use tokio::io;
use tokio::io::AsyncWriteExt;

pub struct Client {
    inner: RealClient,
    scheme: String,
}

#[async_trait]
pub trait Call {
    async fn call(&mut self);
}

#[cfg(unix)]
pub struct RealClient {
    socket_fd: String,
}

#[cfg(windows)]
#[async_trait]
pub struct RealClient {}

const FILE_PATH: &str = "/tmp/wasmship.sock";

impl Client {
    pub fn new(client: RealClient) -> Client {
        Client {
            inner: client,
            scheme: "http".to_string(),
        }
    }

    pub fn init() -> Client {
        Self::new(RealClient::new())
    }

    pub fn into_inner(self) -> RealClient {
        self.inner
    }
}

#[async_trait]
impl Call for Client {
    async fn call(&mut self) {
        self.inner.call().await;
    }
}

#[cfg(unix)]
#[async_trait]
impl Call for RealClient {
    async fn call(&mut self) {
        let client = hyper::Client::unix();
        let url = Uri::new("/tmp/wasmship.sock", "/").into();
        let mut response = client.get(url).await.unwrap();
        while let Some(next) = response.data().await {
            let chunk = next.unwrap();
            io::stdout().write_all(&chunk).await.unwrap();
        }
    }
}

#[cfg(unix)]
impl RealClient {
    fn new() -> RealClient {
        RealClient {
            socket_fd: FILE_PATH.to_string(),
        }
    }
}

#[cfg(windows)]
#[async_trait]
impl Call for RealClient {
    async fn call(&mut self) {
        unimplemented!("named pipe not support now.")
    }
}

#[cfg(windows)]
impl RealClient {
    fn new() -> RealClient {
        RealClient {}
    }
}
