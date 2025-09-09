FROM rust:1.89-slim

WORKDIR /usr/src/game-engine

RUN cargo install cargo-watch

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]
