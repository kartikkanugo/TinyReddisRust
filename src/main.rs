// Uncomment this block to pass the first stage
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    println!("Logs will appear here");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        handle_connection(stream).await;
    }
}

async fn handle_connection(stream: TcpStream) {
    let response = "+PONG\r\n";
    loop {
        if let Ok(()) = stream.readable().await {
            let mut buffer = Vec::with_capacity(1000);
            match stream.try_read_buf(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    stream.try_write(response.as_bytes()).unwrap();
                }
                Err(_) => return,
            }
        }
    }
}
