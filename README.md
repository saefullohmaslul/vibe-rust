# Vibe Rust

A Rust web application built with Axum framework.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd vibe-rust
```

2. Install dependencies:
```bash
cargo build
```

## Development

### Build the project
```bash
cargo build
```

### Run the application
```bash
cargo run
```

### Watch mode (auto-restart on changes)
```bash
cargo watch -q -c -w src/ -x run
```

**Note**: You need to install `cargo-watch` first if you haven't already:
```bash
cargo install cargo-watch
```

## API Endpoints

- `GET /health` - Health check endpoint

## Environment Variables

Create a `.env` file in the root directory for environment configuration.

## Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `sqlx` - Database toolkit
- `chrono` - Date and time handling
- `uuid` - UUID generation
- `dotenvy` - Environment variable loading
- `tower-http` - HTTP middleware (CORS)