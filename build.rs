fn main() {
    tonic_build::compile_protos("proto/pingservice.proto").unwrap();
}
