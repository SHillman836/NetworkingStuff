use tokio::net::TcpStream;
use protocol::TcpPlus;


pub fn read_tcp_stream(mut stream: TcpStream) -> {
    message_vec = TcpPlus::read_message(&stream).await?;
    






}