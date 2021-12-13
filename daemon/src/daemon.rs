use std::sync::atomic::{AtomicUsize, Ordering};
use std::{fs, path::Path};
use tokio::runtime::Builder;

const PHRASE: &str = "It's a Unix system. I know this.";

pub fn run() {
    Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("tokio-worker-{}", id)
        })
        .enable_all()
        .build()
        .unwrap()
        .block_on(server())
}

async fn server() {
    #[cfg(unix)]
    uds_server();
    #[cfg(windows)]
    unimplemented!("windows has not implement!")
}

#[cfg(unix)]
async fn uds_server() {
    use hyper::{
        service::{make_service_fn, service_fn},
        Body, Response, Server,
    };
    use hyperlocal::UnixServerExt;

    let path = Path::new("/tmp/wasmship.sock");

    if path.exists() {
        fs::remove_file(path).unwrap();
    }

    let make_service = make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(|_req| async {
            Ok::<_, hyper::Error>(Response::new(Body::from(PHRASE)))
        }))
    });

    let server = Server::bind_unix(path).unwrap();
    server.serve(make_service).await.unwrap();
}
