use std::net::{TcpListener, TcpStream};

use std::fs::File;
use std::io::{self, Read, Write};

// extern crate lazy_static;
// use lazy_static::lazy_static;
// lazy_static! {
//     pub static ref IPV4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123).into();
// }

#[path = "init_message.rs"]
mod init_message;
#[path = "udp_utils.rs"]
mod udp_utils;

pub fn handle_sender_side() -> std::io::Result<()> {
    println!("Sender is initialized!!");

    let client_addr = udp_utils::udp_multicast_get_client().expect("");

    let socket = TcpListener::bind("0.0.0.0:8080")?;

    for stream in socket.incoming() {
        match stream {
            Ok(stream) => {
                if let Ok(peer_addr) = stream.peer_addr() {
                    println!("Connected to: {}", peer_addr);
                    if client_addr.contains(&peer_addr.ip()) {
                        let thread_handler = std::thread::spawn(|| handle_reciever_tcp(stream));
                        let _ = thread_handler.join();

                        break;
                    } else {
                        eprintln!("[ERROR] Trying to connect unknown device!");
                    }
                } else {
                    println!("Unable to get peer address");
                }
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to connect to the reciever {}", e);
            }
        }
    }

    Ok(())
}

fn send_file(stream: &mut TcpStream, mut file: File) -> Result<(), io::Error> {
    let mut buffer = [0; 8192]; // Use a reasonable buffer size

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }

        stream.write_all(&buffer[..bytes_read])?;
    }

    println!("File sent successfully");
    Ok(())
}

fn handle_reciever_tcp(mut stream: TcpStream) {
    let file = File::open("large_file.txt").expect("Failed to open file");

    let msg = init_message::init_message_create("test.txt".to_string(), 249);

    let msg_serialized = init_message::init_message_serialize(&msg);

    stream
        .write_all(&msg_serialized.as_bytes())
        .expect("[ERROR] Cannot send the message!");

    send_file(&mut stream, file).expect("Failed to send file");
}
