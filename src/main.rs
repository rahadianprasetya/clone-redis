use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Menerima Koneksi Baru");
                handle_koneksi(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_koneksi(stream: &mut TcpStream) {
    // Create a buffer to host incoming data.
    let mut buffer = [0; 512];

    // Read from the stream into the buffer.
    let read_bytes = stream.read(&mut buffer).unwrap();
    if read_bytes == 0 {
        return;
    }
    // Hardcoded response.
    let response = "+PONG\r\n";

    // Write the response to the stream.
    stream.write_all(response.as_bytes()).unwrap();

    // Make sure the stream is flushed.
    stream.flush().unwrap();
}
