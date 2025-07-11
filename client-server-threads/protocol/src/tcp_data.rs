use crate::fb_schema_generated::tcp_plus_data::{
    root_as_tcp_plus_data,
    TcpPlusData,
    TcpPlusDataArgs,
};
use flatbuffers::FlatBufferBuilder;
use std::io::{self, ErrorKind};


pub struct TcpData {
    pub client:  u32,
    pub message: String,
    pub repeat:  u32,
}


impl TcpData {
    pub fn new(client: u32, message: &str, repeat: u32) -> Self {
        Self {
            client,
            message: message.to_string(),
            repeat,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::with_capacity(1024);
        let msg_off  = builder.create_string(&self.message);
        let tcp_off  = TcpPlusData::create(&mut builder, &TcpPlusDataArgs {
            client:  self.client,
            message: Some(msg_off),
            repeat:  self.repeat,
        });
        builder.finish(tcp_off, None);
        builder.finished_data().to_vec()
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, io::Error> {
        let data = root_as_tcp_plus_data(buf)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

        let msg_str = data
            .message()
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "missing message"))?;

        Ok(TcpData::new(data.client(), msg_str, data.repeat()))
    }
}
