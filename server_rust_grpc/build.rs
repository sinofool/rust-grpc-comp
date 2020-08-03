fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/generated")
        .include("../proto")
        .input("../proto/user_service.proto")
        .input("../proto/user_model.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}
