# Stream Sync 🚀

A WebSocket-based real-time communication system with a TypeScript server and Rust client.

## Project Structure 📁

```
stream-sync/
├── client/                 # TypeScript WebSocket Server
│   ├── src/
│   │   └── index.ts       # Server implementation
│   ├── package.json
│   └── tsconfig.json
└── src/                   # Rust WebSocket Client
    └── main.rs           # Client implementation
```

## Features ✨

### TypeScript WebSocket Server
- 🔌 Handles multiple client connections
- 📡 Echoes received messages back to clients
- 🛑 Graceful shutdown with Ctrl+C
- 🎨 Colored console output for better visibility

### Rust WebSocket Client
- 🔄 Automatic reconnection with configurable retries
- ⏱️ Sends periodic messages every 5 seconds
- 🛡️ Robust error handling with custom error types
- 🎯 Connection status feedback with colored output
- 🔌 Graceful disconnection with Ctrl+C

## Setup & Running 🚀

### Prerequisites
- Node.js and npm for TypeScript server
- Rust and Cargo for Rust client

### TypeScript Server
1. Navigate to the client directory:
```bash
cd client
```

2. Install dependencies:
```bash
npm install
```

3. Start the server:
```bash
npm run dev
```

The server will start on `ws://127.0.0.1:8080`

### Rust Client
1. From the project root, build and run the client:
```bash
cargo run
```

The client will:
- Attempt to connect to the server
- Retry up to 5 times if connection fails
- Show connection status with colored output
- Send periodic messages when connected

## Error Handling 🛡️

The Rust client includes robust error handling:
- Connection retry logic (5 attempts, 2-second delay)
- Custom error types for different failure scenarios
- Clear visual feedback for connection status
- Graceful shutdown on both success and failure

## Development 🛠️

### Branches
- `main`: Stable release
- `feature/rust-websocket-client`: Rust client implementation

## License 📄

MIT License - See LICENSE file for details

## Author ✍️

Dr. Sabry Awad <dr.sabry1997@gmail.com>
