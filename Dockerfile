FROM rust AS BUILDER

RUN apt-get update && apt-get install -y cmake golang protobuf-compiler protobuf-compiler-grpc

WORKDIR /code
ADD .cargo /code/.cargo
ADD Cargo.toml /code/Cargo.toml
ADD Cargo.lock /code/Cargo.lock
ADD vendor /code/vendor
ADD build.rs /code/build.rs
ADD proto /code/proto
ADD src /code/src

RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=BUILDER /code/target/release/mme /app/mme
ADD log4rs.yml /app/log4rs.yml
EXPOSE 50051

CMD /app/mme
