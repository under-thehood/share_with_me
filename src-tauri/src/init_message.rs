use std::net::TcpStream;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitMsg {
    pub file_name: String,
    pub file_size: i32,
}

pub fn init_message_create(file_name: String, file_size: i32) -> InitMsg {
    // let mut msg = init_message::InitMsg::default();
    let msg = InitMsg {
        file_name: file_name,
        file_size: file_size,
    };
    msg
}

pub fn init_message_serialize(msg: &InitMsg) -> String {
    serde_json::to_string(msg).expect("[ERROR] Cannot Serialize the data")
}

pub fn init_message_deserialize(stream: &TcpStream) -> InitMsg {
    let mut de = serde_json::Deserializer::from_reader(stream);
    let msg = InitMsg::deserialize(&mut de).expect("[ERROR] Cannot Deserialize the data!");

    msg
}
