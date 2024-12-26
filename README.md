# Stream Sync 🔄

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Node.js](https://img.shields.io/badge/Node.js-18.0+-green.svg)](https://nodejs.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight and efficient WebSocket implementation featuring a high-performance Rust server and a Node.js client. This project demonstrates real-time bidirectional communication between a Rust backend and Node.js frontend.

## ✨ Features

- **High Performance**: Built with Rust's async runtime (Tokio)
- **Real-time Communication**: Bidirectional WebSocket messaging
- **Simple Echo Server**: Demonstrates basic WebSocket functionality
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Easy to Extend**: Clean architecture for adding custom message handling

## 🚀 Quick Start

### Prerequisites

- Rust (1.75 or higher)
- Node.js (18.0 or higher)
- npm (Node Package Manager)

### Rust Server Setup

1. Clone the repository:
```bash
git clone https://github.com/sabry-awad97/stream-sync.git
cd stream-sync
```

2. Run the Rust server:
```bash
cargo run
```

The server will start on `ws://127.0.0.1:8080`

### Node.js Client Setup

1. Navigate to the client directory:
```bash
cd client
```

2. Install dependencies:
```bash
npm i
```

3. Start the client:
```bash
npm run dev
```

## 🛠️ Technical Details

### Server (Rust)
- Built with `tokio` for async runtime
- Uses `tokio-tungstenite` for WebSocket implementation
- Handles multiple concurrent connections
- Echo server implementation for testing

### Client (Node.js)
- Simple WebSocket client implementation
- Demonstrates connection and message exchange
- Console logging for received messages

## 📝 Usage

1. Start the Rust server first
2. Run the Node.js client
3. The client will automatically connect to the server
4. Any messages sent from the client will be echoed back by the server
5. All messages are logged to the console

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Open issues for bugs or feature requests
- Submit pull requests
- Improve documentation
- Share feedback

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Dependencies

### Rust Dependencies
- tokio = "1.35.0"
- tokio-tungstenite = "0.26.1"
- futures-util = "0.3.29"

### Node.js Dependencies
- ws (WebSocket client)
