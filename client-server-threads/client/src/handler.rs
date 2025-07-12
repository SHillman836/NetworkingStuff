use protocol::{
    protocol::TcpPlus,
    tcp_data::TcpData,
};
use rand::Rng;
use tokio::{net::TcpStream, task};
use std::io;
use tokio::io::AsyncWriteExt;


// Top-level: create N tasks, each runs an independent TCP workflow.
// They all run concurrently as each task is spawned before they're awaited.
pub async fn spawn_connection_tasks(n_conns: u32) {
    let mut handles = Vec::new();

    for client_id in 1..=n_conns {
        handles.push(task::spawn(async move {
            if let Err(e) = run_connection(client_id).await {
                eprintln!("Client #{client_id} Error: {e}");
            }
        }));
    }

    // await all tasks
    for h in handles {
        let _ = h.await;
    }
}


// One connection = connect ➜ send loop ➜ read reply loop ➜ shutdown.
async fn run_connection(client_id: u32) -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3000").await?;
    let how_many = rand::thread_rng().gen_range(1..=5);

    for repeat in 1..=how_many {
        let req_bytes = build_request(client_id, repeat);
        TcpPlus::write_message(&mut stream, &mut req_bytes.clone()).await?;
        println!("Client #{client_id} - Sent msg iteration: #{repeat}");

        let resp = read_response(&mut stream).await?;
        println!(
            "Client: #{client_id} - Reply Iteration: {} - Message: {}",
            resp.repeat, resp.message
        );
    }

    // tell server we're done
    stream.shutdown().await?;
    Ok(())
}


// Build + serialize one outbound TcpData request.
fn build_request(client_id: u32, repeat: u32) -> Vec<u8> {
    let msg  = format!("Ping {}", repeat);
    let data = TcpData::new(client_id, &msg, repeat);
    data.serialize()
}


// Read one reply and deserialize it.
async fn read_response(stream: &mut TcpStream) -> io::Result<TcpData> {
    let raw = TcpPlus::read_message(stream).await?;
    TcpData::deserialize(&raw)
}
