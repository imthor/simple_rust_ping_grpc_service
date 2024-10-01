# simple_rust_ping_grpc_service 

## build 

### prerequisites 
```sudo apt install protobuf-compiler```

```
cargo build --release --bin cli
```

## usage

### server
```
♪ ./target/release/cli server start
Starting server: None
PingServer listening on [::1]:50051
Received PingRequest: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: PingRequest { message: "hello" }, extensions: Extensions }
Received PingRequest: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: PingRequest { message: "whats up bro" }, extensions: Extensions }

```

### client
```
♪ ./target/release/cli client -m "hello"
You've selected Client with message: hello, server: [::1], port: 50051
Response from server: "Pong: hello"
♪ ./target/release/cli client -m "whats up bro"
You've selected Client with message: whats up bro, server: [::1], port: 50051
Response from server: "Pong: whats up bro"
♪ 
```
