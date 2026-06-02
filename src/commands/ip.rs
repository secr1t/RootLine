use std::net::{UdpSocket, TcpStream};
use std::io::{Read, Write};

pub fn run() {
    println!("Local IP: {}", get_local_ip());
    println!("Public IP: {}", get_public_ip().unwrap_or("unknown".to_string()));
}

fn get_local_ip() -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap();

    socket.local_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or("unknown".to_string())
}

fn get_public_ip() -> Option<String> {
    let mut stream = TcpStream::connect("api.ipify.org:80").ok()?;

    let request = "GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n";
    stream.write_all(request.as_bytes()).ok()?;

    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;

    response.lines().last().map(|s| s.to_string())
}
