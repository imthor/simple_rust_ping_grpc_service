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

    /// Server-related commands
    Server(ServerArgs),

    /// Send a ping to the server
    /// Client command to send a ping message
    Client {
        #[arg(short, long, required = true)]
        message: String,

        /// Optional server address
        #[arg(short = 's', long, default_value = "[::1]")]
        server_address: String,

        /// Optional port number
        #[arg(short = 'p', long, default_value = "50051")]
        port: u16,
    },

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
    /// Check the status of the server
    Status {
        name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Client { message, server_address, port  } => {
            println!("You've selected Client with message: {message}, server: {server_address}, port: {port}");
            grpc::client::send_ping(message, server_address, port).await?; // Call the send_ping function from the client module
        }
        Commands::Server(server) => match server.command {
            ServerCommands::Start { name } => {
                println!("Starting server: {:?}", name);
                grpc::server::start_server().await?; // Call the start_server function from the server module
            }
            ServerCommands::Status { name } => {
                println!("You've selected Status: {:?}", name);
                // Add logic to handle status
            }
        },
    }

    Ok(())
}