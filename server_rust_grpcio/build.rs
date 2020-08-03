extern crate protoc_grpcio;

fn main() {
    let proto_root = "../proto";
    let generated_root = "src/generated";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["user_service.proto", "user_model.proto"],
        &[proto_root],
        &generated_root,
        None
    ).expect("Failed to compile gRPC definitions!");
}
