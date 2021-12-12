extern crate hyper;

use std::os::unix::net::UnixListener;
use hyper::http::{Request, Response};
use hyper::Body;
use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};

#[tokio::main]
async fn main() {
    let unix_listener = UnixListener::bind("/var/wasmship.sock")?;
    let acceptor = hyper::server::accept::from_stream(unix_listener.incoming());
    let server = Server::builder(acceptor);
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });
    server.serve(make_svc).await?;
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}