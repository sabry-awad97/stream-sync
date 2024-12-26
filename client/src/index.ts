import { WebSocket, WebSocketServer } from "ws";

class WebSocketServerApp {
  private wss: WebSocketServer;
  private clients: Set<WebSocket>;
  private isShuttingDown: boolean;

  constructor(private readonly port: number) {
    this.wss = new WebSocketServer({ port });
    this.clients = new Set();
    this.isShuttingDown = false;
    this.setupShutdown();
  }

  public start(): void {
    console.log("\n🚀 Starting WebSocket server...");
    console.log(`✨ Server is listening on ws://127.0.0.1:${this.port}`);
    console.log("👋 Press Ctrl+C to shutdown gracefully\n");

    this.wss.on("listening", () => {
      console.log("🎯 WebSocket server is ready for connections");
    });

    this.wss.on("connection", (ws: WebSocket, req) => {
      this.handleConnection(ws, req);
    });

    this.wss.on("error", (error: Error) => {
      console.error("❌ Server error:", error.message);
    });
  }

  private handleConnection(ws: WebSocket, req: any): void {
    const clientAddress = req.socket.remoteAddress;
    console.log("📡 New connection from:", clientAddress);

    this.clients.add(ws);
    console.log(`🌟 Active connections: ${this.clients.size}`);

    ws.on("message", (message: Buffer) => {
      try {
        const messageStr = message.toString();
        console.log("📩 Received:", messageStr);

        // Echo the message back
        ws.send(messageStr);
        console.log("📤 Sent:", messageStr);
      } catch (error) {
        console.error("❌ Error processing message:", (error as Error).message);
      }
    });

    ws.on("close", () => {
      this.clients.delete(ws);
      console.log("🔌 Client disconnected");
      console.log(`🌟 Active connections: ${this.clients.size}`);
    });

    ws.on("error", (error: Error) => {
      console.error("❌ Client error:", error.message);
    });
  }

  private setupShutdown(): void {
    process.on("SIGINT", async () => {
      if (this.isShuttingDown) return;
      this.isShuttingDown = true;

      console.log("\n\n🛑 Initiating graceful shutdown...");

      // Close all client connections
      for (const client of this.clients) {
        client.close();
      }

      // Wait for connections to close
      await new Promise((resolve) => setTimeout(resolve, 1000));

      // Close the server
      this.wss.close(() => {
        console.log("👋 Server shut down gracefully");
        process.exit(0);
      });
    });
  }
}

// Start the server
const server = new WebSocketServerApp(8080);
server.start();
