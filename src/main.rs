// Uncomment this block to pass the first stage
use redis_starter_rust::thread_pool::ThreadPool;
use redis_starter_rust::resp::{build_response, handle_command};
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
                    handle_command(&_stream);
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}
