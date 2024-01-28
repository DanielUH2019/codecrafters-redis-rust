
use std::io::prelude::*;
use std::net::TcpStream;

// #[derive(Clone)]
// enum DataType {
//     RedisString(usize),
//     RedisArray(usize),
//     Data()
// }

pub fn handle_command(mut stream: &TcpStream) {
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
                        let response = build_response(request_by_lines);
                        stream.write_all(&response).unwrap();
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
                        let response = build_response(request_by_lines);
                        stream.write_all(&response).unwrap();
                        continue;
                    }
                    break;
                }
                // println!("Request: {}", request);
            }
            Err(e) => {
                println!("Error: {}", e);
                stream.flush().unwrap();
                break;
            }
            
        }
        // println!("end loop");
    };
}
pub fn build_response(lines: Vec<String>) -> Vec<u8> {
    // let array_size = parse_array_size(&lines[0]);
    // assert_eq!(array_size, lines.len());
    let command = parse_command(&lines[2]).unwrap();
    println!("Command: {}", command);
    if command == "ping" {
        return b"+PONG\r\n".to_vec();
    }
    if command == "echo" {
        let data = &lines[4];
        let data_size = data.chars().count();
        // let data = data.as_bytes().to_vec();
        // $N\r\n<message>\r\n
        return format!("${}\r\n{}\r\n", data_size, data).as_bytes().to_vec();
    } else {
        panic!("Not implemented command")
    }
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
        _ => None,
    }
}
