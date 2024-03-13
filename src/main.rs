// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs will appear here");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new connection done");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error is {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = "+PONG\r\n";
    loop {
        let mut buffer = [0; 1024];
        let bytesread = stream.read(&mut buffer).expect("failed to read stream");
        if bytesread == 0 {
            break;
        }

        stream.write_all(response.as_bytes()).unwrap();
    }
}
