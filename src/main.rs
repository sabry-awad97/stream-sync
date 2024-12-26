use colored::*;
use futures_util::{SinkExt, StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
    time::sleep,
};
use tokio_tungstenite::accept_async;

#[derive(Debug)]
struct ServerState {
    active_connections: i32,
}

impl ServerState {
    fn new() -> Self {
        Self {
            active_connections: 0,
        }
    }

    fn increment_connections(&mut self) {
        self.active_connections += 1;
        println!(
            "{} New connection established. Active connections: {}",
            "📡".green(),
            self.active_connections.to_string().yellow()
        );
    }

    fn decrement_connections(&mut self) {
        self.active_connections -= 1;
        println!(
            "{} Connection closed. Active connections: {}",
            "🔌".yellow(),
            self.active_connections.to_string().yellow()
        );
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    state: Arc<Mutex<ServerState>>,
    mut shutdown_rx: broadcast::Receiver<()>,
) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!(
                "{} Failed to accept WebSocket connection: {}",
                "❌".red(),
                e.to_string().red()
            );
            return;
        }
    };

    {
        let mut state = state.lock().await;
        state.increment_connections();
    }

    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            message = read.next() => {
                match message {
                    Some(Ok(msg)) => {
                        println!(
                            "{} Received message: {}",
                            "📩".cyan(),
                            msg.to_string().bright_blue()
                        );
                        if let Err(e) = write.send(msg).await {
                            eprintln!(
                                "{} Error sending message: {}",
                                "❌".red(),
                                e.to_string().red()
                            );
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!(
                            "{} Error receiving message: {}",
                            "❌".red(),
                            e.to_string().red()
                        );
                        break;
                    }
                    None => {
                        println!("{} Client disconnected", "🔄".yellow());
                        break;
                    }
                }
            }
            // Handle shutdown signal
            _ = shutdown_rx.recv() => {
                println!("{} Received shutdown signal, closing connection...", "🛑".red());
                break;
            }
        }
    }

    {
        let mut state = state.lock().await;
        state.decrement_connections();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{} Starting WebSocket server...", "🚀".bright_green());

    // Create a TCP listener
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!(
        "{} Server is listening on {}",
        "✨".bright_cyan(),
        "ws://127.0.0.1:8080".bright_blue()
    );
    println!(
        "{} Press {} to shutdown gracefully\n",
        "👋".bright_yellow(),
        "Ctrl+C".bright_red()
    );

    // Setup shutdown channel
    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let shutdown_tx_clone = shutdown_tx.clone();

    // Setup server state
    let state = Arc::new(Mutex::new(ServerState::new()));
    let running = Arc::new(Mutex::new(true));
    let running_clone = running.clone();

    // Handle Ctrl+C
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                println!(
                    "\n\n{} Initiating graceful shutdown...",
                    "🛑".bright_red().bold()
                );

                // Send shutdown signal
                let _ = shutdown_tx_clone.send(());

                // Set running flag to false
                let mut running = running_clone.lock().await;
                *running = false;

                // Give connections time to close gracefully
                sleep(Duration::from_secs(1)).await;
                println!("{} Goodbye!\n", "👋".bright_yellow());
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!(
                    "{} Error setting up Ctrl+C handler: {}",
                    "❌".red(),
                    e.to_string().red()
                );
            }
        }
    });

    // Accept incoming connections
    while let Ok((stream, addr)) = listener.accept().await {
        // Check if we should continue accepting connections
        let running = running.lock().await;
        if !*running {
            break;
        }
        drop(running);

        println!(
            "{} New connection from: {}",
            "🌟".bright_green(),
            addr.to_string().bright_blue()
        );

        let state = Arc::clone(&state);
        let shutdown_rx = shutdown_tx.subscribe();

        tokio::spawn(async move {
            handle_connection(stream, state, shutdown_rx).await;
        });
    }

    Ok(())
}
