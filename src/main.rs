// Uncomment this block to pass the first stage
use redis_starter_rust::thread_pool::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let pool = ThreadPool::new(6);

    for stream in listener.incoming() {
        pool.execute({
            move || match stream {
                Ok(mut _stream) => {
                    handle_connection(&mut _stream);
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}

fn handle_connection(stream: &mut TcpStream) {
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(size) => {
                println!(
                    "incoming message: {:?}",
                    std::str::from_utf8(&buffer[..size])
                );
                stream.write_all(b"+PONG\r\n").unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                // Handle any error that might occur during read
                break;
            }
        }
    }
}
