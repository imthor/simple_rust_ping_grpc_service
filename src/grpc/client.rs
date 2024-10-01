use pingservice::ping_service_client::PingServiceClient;
use pingservice::PingRequest;

pub mod pingservice {
    tonic::include_proto!("pingservice");
}

// Function to send a message to the gRPC server
pub async fn send_ping(message: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PingServiceClient::connect("http://[::1]:50051").await?;

    // Create a PingRequest message with the provided message
    let request = tonic::Request::new(PingRequest {
        message: message.clone(),
    });

    // Send the request to the server
    let response = client.ping(request).await?;

    println!("Response from server: {:?}", response.into_inner().message);

    Ok(())
}