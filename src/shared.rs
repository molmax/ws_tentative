use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;
use uuid::Uuid;

/// Represents different types of chat messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatMessage {
    Join { username: String },
    Leave { username: String },
    Message { username: String, content: String },
    UserList { users: Vec<String> },
    Error { message: String },
}

/// Represents a connected client
#[derive(Debug, Clone)]
pub struct Client {
    pub id: Uuid,
    pub username: String,
    pub sender: broadcast::Sender<ChatMessage>,
}

/// Type alias for the clients collection
pub type Clients = std::sync::Arc<tokio::sync::Mutex<HashMap<Uuid, Client>>>;

/// Utility functions for message handling
impl ChatMessage {
    /// Create a join message
    pub fn join(username: String) -> Self {
        Self::Join { username }
    }

    /// Create a leave message
    pub fn leave(username: String) -> Self {
        Self::Leave { username }
    }

    /// Create a chat message
    pub fn message(username: String, content: String) -> Self {
        Self::Message { username, content }
    }

    /// Create a user list message
    pub fn user_list(users: Vec<String>) -> Self {
        Self::UserList { users }
    }

    /// Create an error message
    pub fn error(message: String) -> Self {
        Self::Error { message }
    }

    /// Serialize message to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize message from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Utility functions for client management
pub fn create_client(id: Uuid, username: String, sender: broadcast::Sender<ChatMessage>) -> Client {
    Client {
        id,
        username,
        sender,
    }
}

/// Get list of usernames from clients
pub fn get_user_list(clients: &HashMap<Uuid, Client>) -> Vec<String> {
    clients.values().map(|c| c.username.clone()).collect()
}
