mod handler;

use handler::read_tcp_stream;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::future::Future;


#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let socketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(socketAddr).await?;

    loop {
        let (stream, _) = listener.accept().await?;        
        tokio::task::spawn(async move {
            if let Err(e) = read_tcp_stream(stream).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}