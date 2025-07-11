use tokio::net::TcpStream;
use protocol::protocol::TcpPlus;
use protocol::tcp_data::TcpData;
use std::io::{self, ErrorKind};
use rand::Rng;


pub async fn read_tcp_stream(mut stream: TcpStream) -> Result<(), std::io::Error> {
    loop {
        match TcpPlus::read_message(&mut stream).await {
            Ok(msg) => {
                handle_message(msg, &mut stream).await?;
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                println!("Client disconnected");
                break;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

async fn handle_message(message_vec: Vec<u8>, stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let tcp_data = TcpData::deserialize(&message_vec)?;
    message_handler(tcp_data.client, &tcp_data.message, tcp_data.repeat);

    let response = response_conductor(tcp_data.client, tcp_data.repeat);
    let mut response_bytes_vec = response.serialize();
    TcpPlus::write_message(stream, &mut response_bytes_vec).await?;

    Ok(())
}

// Just print out the message
fn message_handler(client: u32, message: &str, repeat: u32) -> () {
    println!("Client {} - Iteration {} - Message: {}", client, repeat, message);
}

fn response_conductor(client: u32, repeat: u32) -> TcpData {
    let mut rng = rand::thread_rng();
    let roll: u32 = rng.gen_range(1..=3);

    let rand_string = if roll == 1 {
        "This is random response 1"
    } else if roll == 2 {
        "This is random response 2"
    } else {
        "This is random response 3"
    };

    return TcpData::new(client, rand_string, repeat);
}