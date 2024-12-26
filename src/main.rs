use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    // Create a TCP listener bound to "127.0.0.1:8080"
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("WebSocket server listening on ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        // Spawn a new task for each connection
        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("Failed to accept websocket");
            println!("New WebSocket connection established");

            let (mut write, mut read) = ws_stream.split();

            // Echo incoming messages
            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => {
                        println!("Received message: {:?}", msg);
                        write.send(msg).await.expect("Failed to send message");
                    }
                    Err(e) => {
                        println!("Error receiving message: {:?}", e);
                        break;
                    }
                }
            }
        });
    }
}
