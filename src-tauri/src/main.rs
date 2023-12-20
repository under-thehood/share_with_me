// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod reciever;
mod sender;
mod udp_utils;
use std::{
    fs::File, io::Read, io::Write, net::IpAddr, net::SocketAddr, net::TcpListener, net::TcpStream,
    sync::Mutex,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref FILENAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}
use tauri::{api::file, Manager};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct RecieverInfo {
    message: String,
}

fn send_file(stream: &mut TcpStream, mut file: File) -> Result<(), std::io::Error> {
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn setup_sender(app: tauri::AppHandle, files: Vec<String>) {
    println!("I was invoked from Js and files are");

    {
        let mut file_names = FILENAMES.lock().unwrap();
        for file in files {
            println!("{}", file);
            file_names.push(file);
        }
    }

    let client = udp_utils::udp_multicast_get_client().expect("[ERROR]");

    app.emit_all(
        "new_reciever",
        RecieverInfo {
            message: client[0].to_string(),
        },
    )
    .unwrap();
}

#[tauri::command]
async fn ask_reciever_permission(receiver_info: RecieverInfo) {
    if let Ok(ip) = receiver_info.message.parse::<IpAddr>() {
        // Create a SocketAddr using the parsed IpAddr and a port (e.g., 8080)
        let socket_addr = SocketAddr::new(ip, 8080);

        // Now you can use the `socket_addr` variable as needed
        println!("SocketAddr: {:?}", socket_addr);

        let mut socket =
            TcpStream::connect(socket_addr).expect("[ERROR] Couldnot connect to the reciever!");
        let msg = "i want to connect!";
        let mut buffer = [0; 40];

        let _ = socket.write_all(msg.as_bytes());

        socket.read(&mut buffer).expect("[ERROR] Cannot read!");

        println!("Accept status is :{:?}", buffer);
        let file_name: String;

        {
            let file_names = FILENAMES.lock().unwrap();
            file_name = file_names[0].clone();
        }

        // println!("Filename : {}", file_name);
        let file = File::open(file_name).expect("Failed to open file");

        let _ = send_file(&mut socket, file);
    } else {
        eprintln!("Error parsing IP address");
    }
}

#[tauri::command]
async fn setup_reciever(app: tauri::AppHandle) {
    println!("[INFO] Receiver is initialized!!");

    let socket = TcpListener::bind("0.0.0.0:8080").expect("[ERROR] Cannot bind the address");

    //[TODO] Add threading for multiple sender
    let server_addr = udp_utils::udp_multicast_announcing_self().expect("[ERROR]");

    for mut stream in socket.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Ok(peer_addr) = stream.peer_addr() {
                    println!("[INFO] {} is trying to connect", peer_addr);

                    if peer_addr.ip() == server_addr.ip() {
                        app.emit_all(
                            "connect_attempt",
                            RecieverInfo {
                                message: "sender".into(),
                            },
                        )
                        .unwrap();

                        app.listen_global("connect_status_response", move |event| {
                            println!("[INFO] connect_status_response {:?}", event.payload());

                            let mut stream = stream.try_clone().expect("[ERROR]");

                            if let Some(status) = event.payload() {
                                let _ = stream.write_all(status.as_bytes());

                                let mut file = File::create(format!("../../{}", "somefile.txt"))
                                    .expect("[ERRR]");
                                let mut buffer = [0; 8192]; // Use a reasonable buffer size

                                loop {
                                    let bytes_received = stream
                                        .read(&mut buffer)
                                        .expect("[ERROR] Cannot read data!");
                                    if bytes_received == 0 {
                                        break; // End of file
                                    }
                                    file.write_all(&buffer[..bytes_received])
                                        .expect("[ERROR] Cannot write to file");
                                }
                            }
                        });
                    }
                } else {
                    println!("[ERROR] Unable to get peer address");
                }
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to connect to the reciever {}", e);
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            setup_sender,
            setup_reciever,
            ask_reciever_permission
        ])
        .run(tauri::generate_context!())
        .expect("[ERROR] Error while running tauri application");
}
