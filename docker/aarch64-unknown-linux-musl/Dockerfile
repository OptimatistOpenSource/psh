FROM ubuntu:noble-20241118.1

RUN apt update && apt install -y curl git musl-tools clang libclang-dev protobuf-compiler rustup

WORKDIR /app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl && cargo clean && cargo build --release --target x86_64-unknown-linux-musl
