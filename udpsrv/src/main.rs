use std::net::UdpSocket;
use std::str;
use std::thread;

fn main() {
    let server_socket = UdpSocket::bind("127.0.0.1:12345")
                           .expect("Could not bind socket");
    loop {
        let mut buf = [0u8; 1024];
        // クロージャの中に移動されるからクローンする。
        let client_socket = server_socket.try_clone().expect("failed to clone socket");

        match server_socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("handling data from {}", src);
                    print!("{}", str::from_utf8(&buf).expect("failed to convert to String"));
                    client_socket.send_to(&buf, src).expect("failed to send response");
                });
            },
            Err(e) => {
                eprintln!("could not recieve a datagram: {}", e);
            }
        }
    }
}
