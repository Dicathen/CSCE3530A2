#![allow(non_snake_case)]
use std::net;
use std::time;

fn main() {
    //argument checking
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} [server|client] <port>", args[0]);
        std::process::exit(1);
    }
    //server branch
    if args[1] == "server" {
        let port = args[2].parse::<u16>()
            .expect("Could not parse port number");
        let socket = init_server(port);
        loop {
            let mut buf = [0; 256];
            let (amt, src) = socket.recv_from(&mut buf)
                .expect("Could not receive data");
            if &buf[..amt] == b"PING" {
                if rand::random::<f32>() < 0.7 {
                    println!("[Client] Received PING from {}", src);
                    socket.send_to(b"PONG", &src)
                        .expect("Could not send data");
                }
                else{
                    println!("[Server] Packet dropped");
                }
            }
        }
    }
    //client branch
    else if args[1] == "client" {
        let port = args[2].parse::<u16>()
            .expect("Could not parse port number");
        let socket = init_client(port);
        //set the read timeout to 2 seconds
        socket.set_read_timeout(Some(time::Duration::from_secs(2)))
            .expect("Could not set read timeout");
        let mut buf = [0; 256];
        for i in 0..10 {
            std::thread::sleep(time::Duration::from_secs(1));
            print!("[Client] #{}, Sending PING ", i + 1);
            socket.send(b"PING")
                .expect("Could not send data");
            match socket.recv(&mut buf) {
                Ok(amt) => {
                    print!("Received {} \n", std::str::from_utf8(&buf[..amt]).unwrap());
                }
                Err(ref e) => if e.kind() == std::io::ErrorKind::WouldBlock {
                    print!("Timed out \n");
                }
                else {
                    eprint!("Error: {:?}", e);
                }
            }
        }
    }
    else {
        eprintln!("Usage: {} [server|client] <port>", args[0]);
        std::process::exit(1);
    }

}

fn init_server(port: u16) -> net::UdpSocket {
    //initialize the server at localhost and the port number
    let socket = net::UdpSocket::bind(net::SocketAddr::from(([127, 0, 0, 1], port)))
        .expect("Could not bind to address");
    println!("Server listening on port {}", port);
    socket
}

fn init_client(port: u16) -> net::UdpSocket {
    //initialize the client at localhost and the port number
    let socket = net::UdpSocket::bind(net::SocketAddr::from(([127, 0, 0, 1], 0)))
        .expect("Could not bind to address");
    socket.connect(net::SocketAddr::from(([127, 0, 0, 1], port)))
        .expect("Could not connect to server");
    socket
}