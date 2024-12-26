use colored::*;
use futures_util::{SinkExt, StreamExt};
use std::{io, time::Duration};
use thiserror::Error;
use tokio::{sync::mpsc, time::sleep};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[derive(Error, Debug)]
enum ClientError {
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

struct WebSocketClient {
    url: String,
    is_running: bool,
    max_retries: u32,
    retry_delay: Duration,
}

impl WebSocketClient {
    fn new(url: &str) -> Result<Self, ClientError> {
        // Validate URL
        let _ = Url::parse(url)?;
        Ok(Self {
            url: url.to_string(),
            is_running: true,
            max_retries: 5,
            retry_delay: Duration::from_secs(2),
        })
    }

    async fn attempt_connect(
        &self,
    ) -> Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        ClientError,
    > {
        let mut retries = 0;
        loop {
            match connect_async(&self.url).await {
                Ok((ws_stream, _)) => {
                    if retries > 0 {
                        println!(
                            "{} Successfully connected after {} {}!",
                            "✅".green(),
                            retries,
                            if retries == 1 { "retry" } else { "retries" }
                        );
                    }
                    return Ok(ws_stream);
                }
                Err(e) => {
                    retries += 1;
                    if retries > self.max_retries {
                        return Err(ClientError::Connection(format!(
                            "Failed to connect after {} retries: {}",
                            self.max_retries, e
                        )));
                    }

                    eprintln!(
                        "{} Connection attempt {} failed: {}",
                        "⚠️".yellow(),
                        retries,
                        e
                    );
                    println!(
                        "{} Retrying in {} seconds... ({}/{})",
                        "🔄".cyan(),
                        self.retry_delay.as_secs(),
                        retries,
                        self.max_retries
                    );

                    sleep(self.retry_delay).await;
                }
            }
        }
    }

    async fn connect(&mut self) -> Result<(), ClientError> {
        println!(
            "\n{} Connecting to WebSocket server...",
            "🚀".bright_green()
        );
        println!(
            "{} Target server: {}",
            "🎯".bright_cyan(),
            self.url.bright_blue()
        );
        println!(
            "{} Press {} to disconnect\n",
            "👋".bright_yellow(),
            "Ctrl+C".bright_red()
        );

        let ws_stream = self.attempt_connect().await?;
        println!("{} Connected to server", "✨".bright_green());

        let (mut write, mut read) = ws_stream.split();
        let (tx, mut rx) = mpsc::channel(32);

        // Clone channel for the message sending task
        let tx_clone = tx.clone();

        // Handle Ctrl+C
        tokio::spawn(async move {
            if let Ok(()) = tokio::signal::ctrl_c().await {
                println!("\n\n{} Initiating graceful shutdown...", "🛑".bright_red());
                let _ = tx_clone.send(None).await;
            }
        });

        // Send periodic messages
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                let message = format!(
                    "Hello from Rust client! Time: {:?}",
                    std::time::SystemTime::now()
                );
                if tx_clone.send(Some(message)).await.is_err() {
                    break;
                }
            }
        });

        // Main event loop
        while self.is_running {
            tokio::select! {
                // Handle incoming messages
                Some(msg) = read.next() => {
                    match msg {
                        Ok(msg) => {
                            println!(
                                "{} Received: {}",
                                "📩".cyan(),
                                msg.to_string().bright_blue()
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "{} Error receiving message: {}",
                                "❌".red(),
                                e.to_string().red()
                            );
                            break;
                        }
                    }
                }
                // Handle outgoing messages
                Some(msg) = rx.recv() => {
                    match msg {
                        Some(text) => {
                            println!("{} Sending: {}", "📤".green(), text.bright_blue());
                            if let Err(e) = write.send(Message::Text(text.into())).await {
                                eprintln!(
                                    "{} Error sending message: {}",
                                    "❌".red(),
                                    e.to_string().red()
                                );
                                break;
                            }
                        }
                        None => {
                            println!("{} Disconnecting from server...", "🔌".yellow());
                            self.is_running = false;
                        }
                    }
                }
            }
        }

        // Cleanup and exit
        sleep(Duration::from_secs(1)).await;
        println!("{} Goodbye!\n", "👋".bright_yellow());
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("\n{} Error: {}\n", "❌".red().bold(), e.to_string().red());
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<(), ClientError> {
    let mut client = WebSocketClient::new("ws://127.0.0.1:8080")?;
    client.connect().await?;
    Ok(())
}
