use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    println!("Logs will appear here");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            println!("In async tokio");
            if let Err(e) = handle_connection(&mut socket).await {
                println!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let response = "+PONG\r\n";
    let mut buffer = Vec::with_capacity(1000);

    loop {
        buffer.clear(); // Clear the buffer before each read operation
        let bytes_read = stream.read(&mut buffer).await?;

        if bytes_read == 0 {
            break; // End of stream
        }
        println!("bytes read {}", bytes_read);

        stream.write_all(response.as_bytes()).await?; // Write response to the stream
    }

    Ok(())
}
