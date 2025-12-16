# OAuth2 Learning Project

## Purpose

A minimal developer-focused usage guide for this workspace.
Goal: learn OAuth2 by implementing an authorization server and a client.

## Quick start

### Prerequisites

- Rust toolchain (`rustup`, `cargo`) installed.

### Build

- Build the entire workspace:

```bash
cargo build --workspace
```

Run (server)

- Run the server crate:

```bash
cargo run --package server
```

- The server currently binds to `127.0.0.1:3000` (see `server/src/main.rs:9`).

### Endpoints

- `GET /` — returns `Hello, World!`

Example:

```bash
curl http://127.0.0.1:3000/
```

