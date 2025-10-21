use crate::shared::ChatMessage;
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// WebSocket chat client implementation
pub struct ChatClient {
    url: String,
    username: String,
}

impl ChatClient {
    /// Create a new chat client
    pub fn new(url: String, username: String) -> Self {
        Self { url, username }
    }

    /// Connect to the server and start the chat session
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔌 Connecting to {}...", self.url);
        
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Send join message
        let join_msg = ChatMessage::join(self.username.clone());
        let join_json = join_msg.to_json()?;
        ws_sender.send(Message::Text(join_json)).await?;

        println!("✅ Connected as {}", self.username);
        println!("💬 Type your messages and press Enter. Type '/quit' to exit.");

        // Handle incoming messages
        let username_clone = self.username.clone();
        tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(chat_msg) = ChatMessage::from_json(&text) {
                            Self::handle_message(chat_msg, &username_clone);
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("🔌 Connection closed by server");
                        break;
                    }
                    Err(e) => {
                        eprintln!("❌ WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Handle user input
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            reader.read_line(&mut line).await?;
            let input = line.trim();

            if input == "/quit" {
                println!("👋 Goodbye!");
                break;
            }

            if !input.is_empty() {
                let message = ChatMessage::message(self.username.clone(), input.to_string());
                let message_json = message.to_json()?;
                ws_sender.send(Message::Text(message_json)).await?;
            }
        }

        Ok(())
    }

    /// Handle incoming chat messages
    fn handle_message(msg: ChatMessage, current_username: &str) {
        match msg {
            ChatMessage::Join { username } => {
                println!("🟢 {} joined the chat", username);
            }
            ChatMessage::Leave { username } => {
                println!("🔴 {} left the chat", username);
            }
            ChatMessage::Message { username, content } => {
                if username != current_username {
                    println!("💬 {}: {}", username, content);
                }
            }
            ChatMessage::UserList { users } => {
                println!("👥 Users online: {}", users.join(", "));
            }
            ChatMessage::Error { message } => {
                println!("❌ Error: {}", message);
            }
        }
    }
}
