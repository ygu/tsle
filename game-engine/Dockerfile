FROM rust:1.89-slim

WORKDIR /usr/src/game-engine

RUN cargo install cargo-watch

RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build
