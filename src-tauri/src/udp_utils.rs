use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

extern crate lazy_static;
use lazy_static::lazy_static;

use std::time::Duration;

lazy_static! {
    pub static ref IPV4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123).into();
}

const PORT: u16 = 7645;

pub fn udp_non_block_send(
    socket: &UdpSocket,
    buf: &[u8],
    addr: &SocketAddr,
) -> std::io::Result<()> {
    loop {
        match socket.send_to(buf, addr) {
            Ok(_) => {
                // Sent successfully
                // Add any additional logic here if needed
                return Ok(());
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Socket not ready, try again later
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }
}

pub fn udp_non_block_recv(
    socket: &UdpSocket,
    buf: &mut [u8],
) -> Result<(usize, SocketAddr), std::io::Error> {
    loop {
        // Receive data from the multicast group
        match socket.recv_from(buf) {
            Ok((size, sender_address)) => {
                // Process received data
                // Add any additional logic here if needed
                let received_data = &buf[..size];
                let received_text = std::str::from_utf8(received_data).unwrap_or("[Not UTF-8]");
                println!("Received from {}: {}", sender_address, received_text);

                // socket.send_to("you are connected".as_bytes(), addr)?;
                return Ok((size, sender_address));
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Socket not ready, try again later
                continue;
            }
            Err(e) => return Err(e.into()),
        }
        // Print the received data
    }
}

pub fn udp_multicast_get_client() -> std::io::Result<Vec<IpAddr>> {
    let multicast_group: SocketAddr = SocketAddr::new(IpAddr::from(*IPV4), PORT);
    let mut known_client: Vec<IpAddr> = Vec::new();
    // Create a UDP socket
    let socket = UdpSocket::bind(multicast_group)?;
    socket.set_nonblocking(true)?;
    // socket.set_read_timeout(Some(Duration::from_millis(100)))?;

    socket.join_multicast_v4(&*IPV4, &Ipv4Addr::UNSPECIFIED)?;
    socket.set_multicast_loop_v4(true)?;

    println!("UDP multicast sender joined group: {}", multicast_group);

    // Buffer to hold incoming data
    let mut buf = [0; 1024];

    let msg = "You are welcome!";

    loop {
        let (_size, client_addr) =
            udp_non_block_recv(&socket, &mut buf).expect("[ERROR] While recieving the data");

        if known_client.contains(&client_addr.ip()) {
            udp_non_block_send(
                &socket,
                "[Trying to connect multiple times] !!".as_bytes(),
                &client_addr,
            )?;
        } else {
            udp_non_block_send(&socket, msg.as_bytes(), &client_addr)?;

            known_client.push(client_addr.ip());
            break;
        }
    }

    Ok(known_client)
}



pub fn udp_multicast_announcing_self() -> std::io::Result<(SocketAddr)> {
    let multicast_group: SocketAddr = SocketAddr::new(IpAddr::V4(*IPV4), PORT);

    let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("[ERROR] Couldn't bind to address");

    udp_socket.set_nonblocking(true)?;
    udp_socket.set_read_timeout(Some(Duration::from_millis(100)))?;

    udp_socket.join_multicast_v4(&*IPV4, &Ipv4Addr::UNSPECIFIED)?;
    udp_socket.set_multicast_loop_v4(true)?;

    println!(
        "[INFO] UDP multicast receiver joined group: {}",
        multicast_group
    );

    let ping_msg = "hi i am heree\n";

    let mut buf = [0; 1024];

    let server_addr = loop {
        udp_non_block_send(&udp_socket, &mut ping_msg.as_bytes(), &multicast_group)
            .expect("[ERROR] while sending data!");

        // Receive data from the multicast group
        let (size, sender_addr) =
            udp_non_block_recv(&udp_socket, &mut buf).expect("[ERROR] While recieving!!");

        if size != 0 {
            break sender_addr;
        }
    };

    drop(udp_socket);

    Ok(server_addr)
}
