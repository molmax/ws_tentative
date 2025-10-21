mod shared;
mod server;
mod client;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "chat")]
#[command(about = "A WebSocket chat application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the chat server
    Server {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Connect to a chat server
    Client {
        /// Server address to connect to
        #[arg(short, long, default_value = "ws://localhost:8080")]
        url: String,
        /// Your username
        #[arg(short, long)]
        username: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server { port } => {
            let server = server::ChatServer::new(port);
            server.run().await?;
        }
        Commands::Client { url, username } => {
            let client = client::ChatClient::new(url, username);
            client.run().await?;
        }
    }

    Ok(())
}