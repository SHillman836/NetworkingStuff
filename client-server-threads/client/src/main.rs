mod handler

use handler::handle;
use std::net::SocketAddr;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use hyper::server::conn::http2;
use hyper::rt::Executor;
use std::future::Future;


[#tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let socketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(socketAddr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = ToioIo::new(stream);
        


    }
    let io = TokioIo::stream()


}