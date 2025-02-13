FROM ubuntu:noble-20241118.1

RUN apt update && apt install -y curl git musl-tools clang libclang-dev protobuf-compiler

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add aarch64-unknown-linux-musl

WORKDIR /app

COPY . .

RUN cargo clean && cargo build --release --target aarch64-unknown-linux-musl
