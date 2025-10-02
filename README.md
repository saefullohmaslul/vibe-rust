# Vibe Rust

A Rust web application built with Axum framework.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- Docker & Docker Compose (for PostgreSQL database)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd vibe-rust
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Start the PostgreSQL database:
```bash
docker-compose up -d postgres
```

4. Install dependencies:
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

Create a `.env` file in the root directory for environment configuration. Key variables include:
- `DATABASE_URL` - PostgreSQL connection string
- `POSTGRES_DATABASE` - Database name
- `POSTGRES_USER` - Database user
- `POSTGRES_PASSWORD` - Database password

## Database Setup

### Using Docker Compose
```bash
# Start PostgreSQL container
docker-compose up -d postgres

# Stop PostgreSQL container
docker-compose down postgres

# View logs
docker-compose logs postgres
```

### Manual PostgreSQL Setup
If you prefer to use a local PostgreSQL installation instead of Docker:
1. Install PostgreSQL on your system
2. Create a database: `createdb rust_axum_sqlx`
3. Update the `DATABASE_URL` in your `.env` file

## Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `sqlx` - Database toolkit (PostgreSQL)
- `chrono` - Date and time handling
- `uuid` - UUID generation
- `dotenvy` - Environment variable loading
- `tower-http` - HTTP middleware (CORS)
- `PostgreSQL` - Relational database (via Docker)