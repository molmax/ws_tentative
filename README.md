## **CURSOR AI TEST**

# WebSocket Chat Application

A terminal-based chat application built with Rust and WebSockets, featuring a clean modular architecture.

## Features

- **Real-time messaging** using WebSockets
- **Multi-user support** with user list management
- **Terminal-based interface** for both server and client
- **Command-line interface** with easy server/client mode selection
- **User join/leave notifications**
- **Error handling** and connection management
- **Modular architecture** with separate modules for client, server, and shared logic

## Installation

1. Make sure you have Rust installed: https://rustup.rs/
2. Install Xcode command line tools: `xcode-select --install`
3. Build the project: `cargo build --release`

## Usage

### Starting the Server

```bash
# Start server on default port 8080
cargo run -- server

# Start server on custom port
cargo run -- server --port 3000
```

### Connecting as a Client

```bash
# Connect to localhost:8080 with username "alice"
cargo run -- client --username alice

# Connect to custom server
cargo run -- client --url ws://192.168.1.100:3000 --username bob
```

## Commands

### Server Commands
- `cargo run -- server` - Start the chat server
- `cargo run -- server --port <port>` - Start server on specific port

### Client Commands
- `cargo run -- client --username <name>` - Connect as a client
- `cargo run -- client --url <url> --username <name>` - Connect to specific server

### Client Interface
- Type messages and press Enter to send
- Type `/quit` to disconnect
- See real-time user join/leave notifications
- View current user list when connecting

## Architecture

The application features a clean modular architecture:

### Project Structure
```
src/
├── main.rs          # CLI interface and application entry point
├── shared.rs        # Shared types, message definitions, and utilities
├── server.rs        # WebSocket server implementation
└── client.rs        # Terminal client implementation
```

### Modules

1. **`shared.rs`** - Common types and utilities:
   - `ChatMessage` enum with all message types
   - `Client` struct for connected users
   - Utility functions for message serialization
   - Type aliases for collections

2. **`server.rs`** - WebSocket server:
   - `ChatServer` struct with connection management
   - Client handling and message broadcasting
   - User list management
   - Connection lifecycle management

3. **`client.rs`** - Terminal client:
   - `ChatClient` struct for user interactions
   - Real-time message display
   - Input handling and message sending
   - Connection management

4. **`main.rs`** - Application entry point:
   - CLI argument parsing
   - Server/client mode selection
   - Clean separation of concerns

## Message Types

- `Join` - User joins the chat
- `Leave` - User leaves the chat  
- `Message` - Chat message from user
- `UserList` - List of currently connected users
- `Error` - Error messages

## Example Session

1. **Terminal 1 (Server):**
   ```bash
   cargo run -- server
   🚀 Chat server running on ws://127.0.0.1:8080
   👤 alice connected from 127.0.0.1:54321
   👤 bob connected from 127.0.0.1:54322
   ```

2. **Terminal 2 (Client 1):**
   ```bash
   cargo run -- client --username alice
   🔌 Connecting to ws://localhost:8080...
   ✅ Connected as alice
   👥 Users online: alice
   💬 Type your messages and press Enter. Type '/quit' to exit.
   ```

3. **Terminal 3 (Client 2):**
   ```bash
   cargo run -- client --username bob
   🔌 Connecting to ws://localhost:8080...
   ✅ Connected as bob
   🟢 alice joined the chat
   👥 Users online: alice, bob
   💬 Type your messages and press Enter. Type '/quit' to exit.
   ```

Now you can chat between the terminals in real-time!
