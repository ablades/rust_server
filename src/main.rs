use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    // create a listener 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

// Read and print data from the connection stream
fn handle_connection(mut stream: TcpStream) {
    let mut buff = [0; 1024];

    stream.read(&mut buff).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buff[..]))
}