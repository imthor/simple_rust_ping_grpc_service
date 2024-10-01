use clap::{Parser, Subcommand, Args};
// Import the gRPC server and client modules from the grpc subdirectory
mod grpc {
    pub mod server;
    pub mod client;
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Test command
    Test {
        name: Option<String>,
    },

    /// Server-related commands
    Server(ServerArgs),
}

#[derive(Args, Debug)]
struct ServerArgs {
    #[command(subcommand)]
    command: ServerCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum ServerCommands {
    /// Start the server
    Start {
        name: Option<String>,
    },

    /// Send a ping to the server
    Client {
        /// The message to send to the server
        message: Option<String>,
    },

    /// Check the status of the server
    Status {
        name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Test { name } => {
            println!("You selected Test: {:?}", name);
        }
        Commands::Server(server) => match server.command {
            ServerCommands::Start { name } => {
                println!("Starting server: {:?}", name);
                grpc::server::start_server().await?; // Call the start_server function from the server module
            }
            ServerCommands::Client { message } => {
                let message = message.unwrap_or_else(|| "Ping".to_string());
                println!("Sending message to server: {:?}", message);
                grpc::client::send_ping(message).await?; // Call the send_ping function from the client module
            }
            ServerCommands::Status { name } => {
                println!("You've selected Status: {:?}", name);
                // Add logic to handle status
            }
        },
    }

    Ok(())
}