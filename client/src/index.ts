import WebSocket from "ws";

const ws = new WebSocket("ws://127.0.0.1:8080");

ws.on("open", function open() {
  console.log("Connected to WebSocket server");

  // Send a test message
  ws.send("Hello from Node.js client!");
});

ws.on("message", (data) => {
  console.log("Received:", data.toString());
});

ws.on("close", () => {
  console.log("Disconnected from WebSocket server");
});

ws.on("error", (err) => {
  console.error("WebSocket error:", err);
});
