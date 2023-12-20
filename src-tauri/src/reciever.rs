use std::net::TcpStream;

// extern crate lazy_static;
// use lazy_static::lazy_static;
use std::fs::File;
use std::io::{Read, Write};

// lazy_static! {
//     pub static ref IPV4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123).into();
// }

#[path = "init_message.rs"]
mod init_message;
#[path = "udp_utils.rs"]
mod udp_utils;

pub fn handle_reciever_side() -> std::io::Result<()> {
    println!("Receiver is initialized!!");

    let mut server_addr = udp_utils::udp_multicast_announcing_self().expect("[ERROR]");

    server_addr.set_port(8080);

    let mut socket =
        TcpStream::connect(server_addr).expect("[ERROR] Couldnot connect to the sender!");

    let msg = init_message::init_message_deserialize(&socket);

    let mut file = File::create(format!("../../{}", msg.file_name))?;
    let mut buffer = [0; 8192]; // Use a reasonable buffer size

    loop {
        let bytes_received = socket.read(&mut buffer)?;
        if bytes_received == 0 {
            break; // End of file
        }

        file.write_all(&buffer[..bytes_received])?;
    }

    println!("File received successfully");
    Ok(())
}
