use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use server::ThreadPool;

fn main() {
    
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {

        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

    }

    println!("Shutting down.");

}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let contents: String = fs::read_to_string("index.html").unwrap();

    let response: String = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();

}