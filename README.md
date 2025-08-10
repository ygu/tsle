# tsle - Tempête sur l'Échiquier

## Game Engine

A basic Rust-based backend server for managing chess-like game sessions, built 
with Axum and Tokio.

### Overview

This project provides a simple HTTP API to simulate creating and playing moves 
in a game session.
It is the starting point for a more complex multi-player game engine tailored 
for "Tempête" — a chess variant with extensible rules.

### Features (Current)

- Create a new game session (`POST /create-game`)
- Play a (simili) move in a game session (`POST /play-move`)
- Basic HTTP server running on port 8080
- JSON API with request/response serialization via Serde

### Technologies Used

- Rust
- [Axum](https://docs.rs/axum) for HTTP server and routing
- [Tokio](https://tokio.rs/) async runtime
- [Serde](https://serde.rs/) for JSON serialization/deserialization
- [UUID](https://docs.rs/uuid) for unique game identifiers
- [Tracing](https://docs.rs/tracing) for structured logging

### Getting Started

#### Prerequisites

- Rust toolchain
- Make

#### Build and Run

```bash
make build
make run
```

The server will start listening on `http://0.0.0.0:8080`.

#### API Endpoints

  - `POST /create-game`
    Creates a new game and returns a JSON response with a unique game ID.
  - `POST /play-move`
    Accepts JSON input with `game_id`, `from`, and `to` fields to play a move in the specified game.

Example JSON payload for /play-move:

```json
{
  "game_id": "some-uuid",
  "from": "e2",
  "to": "e4"
}
```

#### Testing the API with curl

Create a new game:

```bash
curl -X POST http://localhost:8080/create-game
```

Play a move:

```bash
curl -X POST http://localhost:8080/play-move -H "Content-Type: application/json" -d '{"game_id":"<id>","from":"e2","to":"e4"}'
```

### Project Structure

  - `src/main.rs` — Server setup and route mounting
  - `src/routes.rs` — API routes definitions
  - `src/state.rs` — Handlers and game state models

### Next Steps

  - Implement real game logic and state management
  - Replace stub logic with actual chess rules engine
  - Add persistence and multi-player support
