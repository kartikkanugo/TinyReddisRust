// Uncomment this block to pass the first stage
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    println!("Logs will appear here");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let socket = listener.accept().await;
        match socket {
            Ok((stream, _)) => {
                println!("accepted a new connection");
                tokio::spawn(async move {
                    handle_connection(stream).await;
                });
            }
            Err(e) => {
                println!("first loop error {}", e);
            }
        }
    }
}

async fn handle_connection(stream: TcpStream) {
    let response = "+PONG\r\n";
    let mut buffer = Vec::with_capacity(1000);

    loop {
        match stream.try_read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                stream.try_write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("handle connection error: {}", e);
                return;
            }
        }
    }
}
