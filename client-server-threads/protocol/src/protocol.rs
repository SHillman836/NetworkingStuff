use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::convert::TryFrom;


pub struct TcpPlus;

impl TcpPlus {
    // Returns a vector of bytes
    pub async fn read_message(tcp_stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
        let mut header = [0u8; 4];
        tcp_stream.read_exact(&mut header).await?;
        
        let len = u32::from_be_bytes(header) as usize;

        let mut msg = Vec::with_capacity(len);
        msg.resize(len, 0);
        tcp_stream.read_exact(&mut msg).await?;

        Ok(msg)
    }

    // Takes in a vector of bytes for the flatbuffers data
    // These bytes are the raw body, they aren't yet in protocol
    pub async fn write_message(tcp_stream: &mut TcpStream, message: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let byte_len: usize = message.len();
        // Convert safely to u32 (or panic / handle overflow)
        let len_u32 = u32::try_from(byte_len)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput,
                "message too large for 4-byte length header"))?;

        let header: [u8; 4] = len_u32.to_be_bytes();

        tcp_stream.write_all(&header).await?;
        tcp_stream.write_all(message).await?;        

        Ok(())
    }
}