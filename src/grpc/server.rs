use tonic::{transport::Server, Request, Response, Status};
use pingservice::ping_service_server::{PingService, PingServiceServer};
use pingservice::{PingRequest, PingResponse};

pub mod pingservice {
    tonic::include_proto!("pingservice");
}

#[derive(Debug, Default)]
pub struct MyPingService {}

#[tonic::async_trait]
impl PingService for MyPingService {
    async fn ping(
        &self,
        request: Request<PingRequest>, // Incoming request
    ) -> Result<Response<PingResponse>, Status> {
        println!("Received PingRequest: {:?}", request);

        // Create response with "Pong"
        let reply = PingResponse {
            message: format!("Pong: {}", request.into_inner().message),
        };

        Ok(Response::new(reply)) // Send back the response
    }
}

// Mark the start_server function as public
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let ping_service = MyPingService::default();

    println!("PingServer listening on {}", addr);

    Server::builder()
        .add_service(PingServiceServer::new(ping_service))
        .serve(addr)
        .await?;

    Ok(())
}