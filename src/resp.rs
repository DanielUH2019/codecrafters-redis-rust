
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

// #[derive(Clone)]
// enum DataType {
//     RedisString(usize),
//     RedisArray(usize),
//     Data()
// }
pub struct Database {
    db: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Database {
        Database { db: HashMap::new() }    
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.db.get(key)
    }

    fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.db.insert(key.to_owned(), value.to_owned())
    }
}


pub fn handle_command(mut stream: &TcpStream, db: &mut Database) {
    // let mut buffer = [0; 512];
    let mut request = String::new();

    loop {
        let mut buffer = vec![0;512];
        let mut is_data_in_buffer = false;
        // Read the request into the buffer
        // println!("end loop");
        match stream.read(&mut buffer) {
            Ok(bytes_read) => { 
                // println!("Bytes read: {}", bytes_read);
                if bytes_read == 0 {
                    if is_data_in_buffer {
                        let request_by_lines: Vec<String> = request.split_terminator("\r\n").map(|line| line.to_string()).collect();
                        let response = build_response(request_by_lines, db);
                        stream.write_all(response[response.len() - 1].as_slice()).unwrap();
                        continue;
                    }
                    stream.flush().unwrap();
                    break;
                }
                let str_buf = String::from_utf8(buffer[0..bytes_read].to_vec()).unwrap_or("\r\n".to_string());
                request.push_str(&str_buf);
                is_data_in_buffer = true;
                if bytes_read < 512 {
                    if is_data_in_buffer {
                        let request_by_lines: Vec<String> = request.split_terminator("\r\n").map(|line| line.to_string()).collect();
                        let response = build_response(request_by_lines, db);
                        stream.write_all(response[response.len() - 1].as_slice()).unwrap();
                        continue;
                    }
                    stream.flush().unwrap();
                    break;
                }
                
            }
            Err(e) => {
                println!("Error: {}", e);
                stream.flush().unwrap();
                break;
            }
            
        }
        
    };
}
pub fn build_response(lines: Vec<String>, db: &mut Database) -> Vec<Vec<u8>> {
    let array_size = parse_array_size(&lines[0]);
    let mut command_index: usize = 2;
    let mut response: Vec<Vec<u8>> = Vec::new();
    let mut i: usize = 0;
    loop {
        if command_index >= lines.len() {
            break;
        }
        let command = parse_command(&lines[command_index]).unwrap();
        println!("lines: {:?} and command: {} ", lines, command);
        if command == "ping" {
            response.push(b"+PONG\r\n".to_vec());
            command_index += 3;
        } else if command == "echo" {
            let data = &lines[4];
            let data_size = data.chars().count();
            response.push(format!("${}\r\n{}\r\n", data_size, data).as_bytes().to_vec());
            command_index += 5;
        } else if command == "set" {
            let key = &lines[4];
            let value = &lines[6];
            db.set(key, value);
            response.push(b"+OK\r\n".to_vec());
            command_index += 7;
        } else if command == "get" {
            let key = &lines[4];
            let value = db.get(key);
            response.push(format!("+{}\r\n", value.unwrap()).as_bytes().to_vec());
            command_index += 5;
        } else {
            panic!("Not implemented command")
        }
        i += 1;
    }

    response
    
}
pub fn parse_array_size(line: &str) -> usize {
    let mut l = line.chars();
    l.next();
    let size: usize = l.next().unwrap().to_digit(10).unwrap().try_into().unwrap();
    size
}
pub fn parse_command(line: &str) -> Option<String>{
    match line{
        "ping" => Some("ping".to_string()),
        "echo" => Some("echo".to_string()),
        "set" => Some("set".to_string()),
        "get" => Some("get".to_string()),
        _ => None,
    }
}
