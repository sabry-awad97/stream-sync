import WebSocket from "ws";

class WebSocketClient {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private readonly maxReconnectAttempts = 3;
  private readonly reconnectDelay = 2000; // 2 seconds
  private isShuttingDown = false;

  constructor(private readonly url: string) {
    // Setup graceful shutdown
    process.on("SIGINT", async () => {
      await this.handleGracefulShutdown();
    });
  }

  public async connect(): Promise<void> {
    try {
      this.ws = new WebSocket(this.url);
      this.setupEventListeners();
      await this.waitForConnection();
      this.reconnectAttempts = 0;
      console.log("\n🚀 Connected to WebSocket server");
      console.log("👋 Press Ctrl+C to shutdown gracefully\n");
    } catch (error) {
      await this.handleConnectionError(error as Error);
    }
  }

  private setupEventListeners(): void {
    if (!this.ws) return;

    this.ws.on("open", () => {
      this.sendTestMessage();
    });

    this.ws.on("message", (data) => {
      console.log("📩 Received:", data.toString());
    });

    this.ws.on("close", async () => {
      if (this.isShuttingDown) {
        console.log("🔌 Connection closed gracefully");
        return;
      }

      console.log("⚠️ Connection lost");
      await this.attemptReconnect();
    });

    this.ws.on("error", (error) => {
      console.error("❌ WebSocket error:", (error as Error).message);
    });
  }

  private async waitForConnection(): Promise<void> {
    if (!this.ws) throw new Error("WebSocket instance not initialized");

    return new Promise((resolve, reject) => {
      const timeoutId = setTimeout(() => {
        reject(new Error("Connection timeout"));
      }, 5000);

      this.ws!.once("open", () => {
        clearTimeout(timeoutId);
        resolve();
      });

      this.ws!.once("error", (error) => {
        clearTimeout(timeoutId);
        reject(error);
      });
    });
  }

  private async handleConnectionError(error: Error): Promise<void> {
    console.error("❌ Connection error:", error.message);
    await this.attemptReconnect();
  }

  private async attemptReconnect(): Promise<void> {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error("❌ Max reconnection attempts reached. Exiting...");
      process.exit(1);
    }

    this.reconnectAttempts++;
    console.log(
      `🔄 Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`
    );

    await new Promise((resolve) => setTimeout(resolve, this.reconnectDelay));
    await this.connect();
  }

  private sendTestMessage(): void {
    if (!this.ws) return;

    const message = "Hello from TypeScript client!";
    this.ws.send(message);
    console.log("📤 Sent:", message);
  }

  private async handleGracefulShutdown(): Promise<void> {
    if (this.isShuttingDown) return;
    this.isShuttingDown = true;

    console.log("\n\n🛑 Initiating graceful shutdown...");

    if (this.ws) {
      this.ws.close();
      await new Promise((resolve) => setTimeout(resolve, 1000)); // Give time for the close event to be processed
    }

    console.log("👋 Goodbye!");
    process.exit(0);
  }
}

// Start the client
const client = new WebSocketClient("ws://127.0.0.1:8080");
client.connect().catch((error: unknown) => {
  if (error instanceof Error) {
    console.error("❌ Failed to start client:", error.message);
  } else {
    console.error("❌ Failed to start client:", error);
  }
  process.exit(1);
});
