FROM rust:1.89-slim AS builder

WORKDIR /usr/src/game-engine

RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/src/game-engine

COPY --from=builder /usr/src/game-engine/target/release/game-engine ./target/release/game-engine

CMD ["./target/release/game-engine"]
