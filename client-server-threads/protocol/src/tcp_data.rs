extern crate flatbuffers;

use fb_schema_generated::{
    root_as_tcp_plus_data,
    TcpPlusData
};
use flatbuffers::FlatBufferBuilder;


pub struct TcpData {
    client: u32,
    message: String,
    repeat: u32,
}

impl TcpData {
    pub fn new(client: u32, message: &str, repeat: u32) -> Self {
        return Self {
            client: client,
            message: message.to_string(),
            repeat: repeat,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::with_capacity(1024);

        let message = builder.create_string(self.message);
        let tcp_data = TcpPlusData::create(&mut builder, &TcpPlusDataArgs{
            client: self.client,
            message: message,
            repeat: self.repeat
        });
        builder.finish(tcp_data, None);

        return builder.finished_data().to_vec();
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, std::io::Error> {
        let tcp_data = root_as_tcp_plus_data(buf)
            .map_err(|e| { io::Error::new(
                    ErrorKind::InvalidData,
                    format!("FlatBuffer parse error: {}", e),
                )
            })

        let msg_str = data
            .message()
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "missing message"))?;

        
        Ok(TcpData::new(data.client(), msg_str, data.repeat()))
    }
}


