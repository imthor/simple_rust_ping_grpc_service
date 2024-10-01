use pingservice::ping_service_client::PingServiceClient;
use pingservice::PingRequest;

pub mod pingservice {
    tonic::include_proto!("pingservice");
}

// Function to send a message to the gRPC server
pub async fn send_ping(
    message: String,
    address: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {

    // Construct the full server address
    let server_url = format!("http://{}:{}", address, port);

    // Connect to the gRPC server
    let mut client = PingServiceClient::connect(server_url).await?;

    // Create a PingRequest message with the provided message
    let request = tonic::Request::new(PingRequest {
        message: message.clone(),
    });

    // Send the request to the server
    let response = client.ping(request).await?;

    println!("Response from server: {:?}", response.into_inner().message);

    Ok(())
}