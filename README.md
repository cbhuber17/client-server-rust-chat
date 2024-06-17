# Rust Chat Application

A simple TCP server-client chat application written in Rust. This application allows multiple clients to connect to a server and broadcast messages to all connected clients.

## Features

- Concurrent client handling using threads.
- Non-blocking I/O to continuously check for new connections and messages.
- Broadcast messages from one client to all connected clients.
- Graceful handling of client disconnections.

## Project Structure

```
rust-chat-app/
│
├── src/
│ ├── client/
│ │ └── main.rs
│ └── server/
│ └── main.rs
│
├── Cargo.toml
└── README.md
```

## Prerequisites

- Rust (install from [rust-lang.org](https://www.rust-lang.org/))
- Cargo (comes with Rust)

## Getting Started

1. **Clone the repository:**

   ```sh
   git clone https://github.com/cbhuber17/client-server-rust-chat.git
   cd client-server-rust-chat
   ```

2. **Build the project:**

   ```sh
   cargo build --release
   ```

3. **Running the server:**

   Open a terminal and run:

   ```sh
   cargo run --release --bin server
   ```

4. **Running the client:**

   Open another terminal for each client and run:

   ```sh
   cargo run --release --bin client
   ```

## Usage

1. **Start the server:**

   Run the server using the command mentioned above. The server listens on `127.0.0.1:6000`.

2. **Connect clients:**

   Run the client(s) using the command mentioned above. Each client will connect to the server at `127.0.0.1:6000`.

3. **Send messages:**

   - Type a message in the client terminal and press Enter to send it.
   - Type `:quit` (or CTRL+C) to exit the chat.

4. **Receive messages:**

   Messages from other clients will be displayed in each client's terminal.

## Example

1. **Server Output:**

   ```sh
   Client 127.0.0.1:XXXXX connected
   127.0.0.1:XXXXX: "Hello from client 1"
   127.0.0.1:YYYYY: "Hello from client 2"
   ```

2. **Client Output:**

   ```sh
   Write a message:
   Message recv: "[72, 101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 99, 108, 105, 101, 110, 116, 32, 49]"
   Message sent "[72, 101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 99, 108, 105, 101, 110, 116, 32, 50]"
   ```
