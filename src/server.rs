use crate::shared::{ChatMessage, Client, Clients, create_client, get_user_list};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;

/// WebSocket chat server implementation
pub struct ChatServer {
    port: u16,
    clients: Clients,
    broadcast_tx: broadcast::Sender<ChatMessage>,
}

impl ChatServer {
    /// Create a new chat server
    pub fn new(port: u16) -> Self {
        let (tx, _rx) = broadcast::channel(1000);
        Self {
            port,
            clients: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            broadcast_tx: tx,
        }
    }

    /// Start the server and listen for connections
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;

        println!("🚀 Chat server running on ws://{}", addr);

        while let Ok((stream, addr)) = listener.accept().await {
            let clients = self.clients.clone();
            let tx = self.broadcast_tx.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(stream, addr, clients, tx).await {
                    eprintln!("Error handling client: {}", e);
                }
            });
        }

        Ok(())
    }

    /// Handle a client connection
    async fn handle_client(
        stream: TcpStream,
        addr: SocketAddr,
        clients: Clients,
        tx: broadcast::Sender<ChatMessage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let client_id = Uuid::new_v4();
        let mut username = String::new();

        // Wait for the first message to get the username
        if let Some(msg) = ws_receiver.next().await {
            let msg = msg?;
            if let Message::Text(text) = msg {
                if let Ok(chat_msg) = ChatMessage::from_json(&text) {
                    if let ChatMessage::Join { username: name } = chat_msg {
                        username = name.clone();
                        
                        // Create client and add to list
                        let client = create_client(client_id, username.clone(), tx.clone());
                        
                        {
                            let mut clients = clients.lock().await;
                            clients.insert(client_id, client);
                        }

                        // Notify other clients
                        let join_msg = ChatMessage::join(username.clone());
                        let _ = tx.send(join_msg);

                        // Send current user list to the new client
                        let user_list = {
                            let clients = clients.lock().await;
                            get_user_list(&clients)
                        };
                        let user_list_msg = ChatMessage::user_list(user_list);
                        let user_list_json = user_list_msg.to_json()?;
                        ws_sender.send(Message::Text(user_list_json)).await?;

                        println!("👤 {} connected from {}", username, addr);
                    }
                }
            }
        }

        // Handle incoming messages
        let mut rx = tx.subscribe();
        let clients_clone = clients.clone();
        let tx_clone = tx.clone();
        let username_clone = username.clone();

        // Spawn task to handle outgoing messages to this client
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Err(e) = ws_sender.send(Message::Text(msg.to_json().unwrap())).await {
                    eprintln!("Error sending message: {}", e);
                    break;
                }
            }
        });

        // Handle incoming messages from this client
        while let Some(msg) = ws_receiver.next().await {
            let msg = msg?;
            if let Message::Text(text) = msg {
                if let Ok(chat_msg) = ChatMessage::from_json(&text) {
                    match chat_msg {
                        ChatMessage::Message { content, .. } => {
                            let message = ChatMessage::message(username_clone.clone(), content);
                            let _ = tx_clone.send(message);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Client disconnected
        {
            let mut clients = clients_clone.lock().await;
            clients.remove(&client_id);
        }

        let leave_msg = ChatMessage::leave(username_clone);
        let _ = tx_clone.send(leave_msg);

        println!("👋 {} disconnected", username_clone);
        Ok(())
    }
}
