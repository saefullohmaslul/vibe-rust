# Vibe Rust

A modern RESTful API for note management built with Rust, Axum framework, and PostgreSQL. This project demonstrates best practices in web development, including clean architecture, comprehensive error handling, and API documentation.

## 🚀 Features

- **RESTful API**: Complete CRUD operations for notes management
- **Modern Architecture**: Clean modular design with Shaku dependency injection
- **Database Integration**: PostgreSQL with connection pooling and migrations
- **API Documentation**: Auto-generated OpenAPI/Swagger documentation
- **Type Safety**: Compile-time guarantees with Rust's type system
- **Async Performance**: Non-blocking I/O with Tokio runtime
- **Input Validation**: Request validation and type-safe serialization
- **CORS Support**: Cross-origin resource sharing configuration
- **Health Checks**: Application health monitoring endpoint
- **Dependency Injection**: Compile-time DI with trait-based components

## 📋 Prerequisites

- **Rust** (latest stable version)
- **Cargo** (comes with Rust)
- **Docker & Docker Compose** (for PostgreSQL database)
- **PostgreSQL Client** (optional, for manual database management)

## 🛠️ Installation

1. **Clone the repository**:
```bash
git clone <repository-url>
cd vibe-rust
```

2. **Set up environment variables**:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. **Start the PostgreSQL database**:
```bash
docker-compose up -d postgres
```

4. **Run database migrations**:
```bash
make migration:run
# Or manually: sqlx migrate run
```

5. **Install development tools** (optional but recommended):
```bash
make install-deps
```

6. **Install dependencies**:
```bash
cargo build
```

## 🏃‍♂️ Development

### Quick Start
For the fastest way to get started:
```bash
# Complete setup (database + migrations + run)
make start
```

### Development Workflow
```bash
# Build the project
make build

# Run the application
make run

# Development mode with hot reload
make dev

# Run all code quality checks
make check

# Run tests
make test
```

### Alternative Manual Commands
If you prefer to use Cargo directly:
```bash
# Build the project
cargo build

# Run the application
cargo run

# Watch mode (auto-restart on changes)
cargo watch -q -c -w src/ -x run
```

**Note**: You need to install `cargo-watch` first if you haven't already:
```bash
make install-deps
# Or manually: cargo install cargo-watch
```

### Development tools
```bash
# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy -- -D warnings

# Run tests
cargo test

# Create new migration
make migration:create NAME=add_new_field

# Run migrations
make migration:run

# Revert last migration
make migration:revert
```

## 🛠️ Makefile Commands

This project includes a comprehensive Makefile to streamline development workflows. Use `make help` to see all available commands.

### Quick Start
```bash
# Complete setup (database + migrations + run)
make start

# Stop all services
make stop
```

### Development Commands
```bash
make build          # Build the project
make run            # Run the application
make dev            # Run in development mode with hot reload
make test           # Run tests
make test:watch     # Run tests in watch mode
make test:verbose   # Run tests with verbose output
make check          # Run all checks (fmt, clippy, test)
make clean          # Clean build artifacts
make fmt            # Format code
make lint           # Run clippy linter
```

### Database Commands
```bash
make db:up          # Start PostgreSQL container
make db:down        # Stop PostgreSQL container
make db:logs        # Show database logs
make db:shell       # Access database shell
make migration:create NAME=migration_name  # Create new migration
make migration:run  # Run all pending migrations
make migration:revert    # Revert last migration
make migration:info Show migration status
```

### Utility Commands
```bash
make setup          # Complete development environment setup
make install-deps   # Install required tools (cargo-watch, sqlx-cli)
make reset-db       # Reset database (with confirmation)
make health         # Check application health
make docs:open      # Open API documentation in browser
```

### Docker Commands
```bash
make docker:build   # Build Docker image
make docker:run     # Run Docker container
make docker:stop    # Stop Docker containers
```

### Example Workflows
```bash
# New development setup
make setup && make dev

# Testing workflow
make fmt && make lint && make test

# Database reset
make reset-db

# Quick development cycle
make db:up && make migration:run && make dev
```

## 📡 API Documentation

Once the application is running, you can access:

- **Swagger UI**: http://localhost:8080/swagger-ui
- **OpenAPI JSON**: http://localhost:8080/api-docs/openapi.json
- **Health Check**: http://localhost:8080/api/v1/health

## 🔌 API Endpoints

### Health Check
- `GET /api/v1/health` - Application health status

### Notes Management
- `GET /api/v1/notes` - List all notes with pagination
- `POST /api/v1/notes` - Create a new note
- `PUT /api/v1/notes/{id}` - Update an existing note

### API Usage Examples

#### Create a Note
```bash
curl -X POST http://localhost:8080/api/v1/notes \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My First Note",
    "content": "This is the content of my note",
    "is_published": true
  }'
```

#### List Notes
```bash
# Get all notes
curl http://localhost:8080/api/v1/notes

# Get paginated notes
curl "http://localhost:8080/api/v1/notes?page=1&limit=5"
```

#### Update a Note
```bash
curl -X PUT http://localhost:8080/api/v1/notes/{note-id} \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Note Title",
    "content": "Updated content"
  }'
```

## 🗄️ Database Setup

### Using Docker Compose (Recommended)
```bash
# Start PostgreSQL container
docker-compose up -d postgres

# Stop PostgreSQL container
docker-compose down postgres

# View logs
docker-compose logs postgres

# Access database interactively
docker-compose exec postgres psql -U postgres -d rust_axum_sqlx
```

### Manual PostgreSQL Setup
If you prefer to use a local PostgreSQL installation:
1. Install PostgreSQL on your system
2. Create a database: `createdb rust_axum_sqlx`
3. Update the `DATABASE_URL` in your `.env` file
4. Run migrations: `sqlx migrate run`

### Database Schema
The application uses the following main table:

```sql
CREATE TABLE notes (
    id CHAR(36) PRIMARY KEY NOT NULL,           -- UUID v4
    title VARCHAR(255) NOT NULL UNIQUE,         -- Unique constraint
    content TEXT NOT NULL,                      -- Long-form content
    is_published BOOLEAN NOT NULL DEFAULT FALSE, -- Publication status
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, -- Creation time
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP  -- Last update
);
```

## ⚙️ Environment Variables

Create a `.env` file in the root directory. Key variables include:

```bash
# Database Configuration
POSTGRES_DATABASE=rust_axum_sqlx
POSTGRES_USER=postgres
POSTGRES_PASSWORD=12345678
POSTGRES_ROOT_PASSWORD=12345678

# Connection String
DATABASE_URL=postgresql://postgres:12345678@127.0.0.1:5432/rust_axum_sqlx
```

## 🏗️ Project Structure

```
vibe-rust/
├── src/
│   ├── main.rs                 # Application entry point + Shaku DI setup
│   ├── infrastructure/         # Infrastructure layer
│   │   ├── mod.rs             # Infrastructure module declaration
│   │   └── database.rs        # Database connection and pooling (Shaku component)
│   ├── models/                 # Data models and schemas
│   │   ├── mod.rs             # Models module declaration
│   │   └── model.rs           # Database models and response schemas
│   └── modules/               # Business logic modules
│       ├── mod.rs             # Modules module declaration
│       ├── commons/           # Common utilities and health checks
│       │   ├── mod.rs         # Commons module declaration
│       │   └── handler.rs     # Health check handlers + routing
│       └── notes/             # Notes management module
│           ├── mod.rs         # Notes module declaration + DTOs + Shaku module
│           ├── handler.rs     # HTTP request handlers
│           ├── service.rs     # Business logic layer (Shaku component)
│           └── repository.rs  # Data access layer (Shaku component)
├── migrations/                # Database migration files
├── docs/                      # Documentation
│   ├── MODULE.md             # Module documentation
│   ├── ARCHITECTURE.md       # System architecture documentation
│   └── DEPENDENCY.md         # Dependency documentation
├── docker-compose.yml         # Docker configuration
├── Makefile                   # Development commands and utilities
├── Cargo.toml                 # Rust dependencies
├── .env.example               # Environment variables template
└── README.md                  # This file
```

## 📚 Documentation

- **[Module Documentation](docs/MODULE.md)** - Detailed explanation of all modules
- **[Architecture Documentation](docs/ARCHITECTURE.md)** - System architecture and design patterns
- **[Dependency Documentation](docs/DEPENDENCY.md)** - Complete dependency overview and usage
- **API Documentation** - Interactive Swagger UI at `/swagger-ui`

## 🧪 Testing

### Using Makefile (Recommended)
```bash
# Run all tests
make test

# Run tests in watch mode
make test:watch

# Run tests with verbose output
make test:verbose

# Run tests with cargo directly
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in release mode
cargo test --release
```

### Testing Workflow
```bash
# Run all code quality checks including tests
make check

# Format, lint, then test
make fmt && make lint && make test
```

## 🔧 Dependencies

### Core Dependencies
- **`axum`** - Modern, ergonomic web framework built on Tokio
- **`tokio`** - Asynchronous runtime for Rust
- **`sqlx`** - Async SQL toolkit with compile-time verification
- **`serde`** - Serialization/deserialization framework
- **`uuid`** - UUID generation and parsing
- **`chrono`** - Date and time handling
- **`shaku`** - Compile-time dependency injection framework
- **`async-trait`** - Async trait support for DI interfaces

### API Documentation
- **`utoipa`** - OpenAPI specification generation
- **`utoipa-swagger-ui`** - Interactive API documentation

### Configuration & Utilities
- **`dotenvy`** - Environment variable management
- **`tower-http`** - HTTP middleware (CORS)

### Database
- **`PostgreSQL`** - Primary database with Shaku-managed connection pooling

## 🚀 Deployment

### Development
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop all services
docker-compose down
```

### Production Considerations
- Use environment-specific configurations
- Implement proper secret management
- Set up SSL/TLS termination
- Configure database backups
- Monitor application health and performance
- Use proper logging and monitoring

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔍 Architecture Highlights

- **Clean Architecture**: Clear separation between presentation, business logic, and data access layers
- **Shaku Dependency Injection**: Compile-time DI with trait-based components
- **Repository Pattern**: Abstract data access with testable interfaces
- **Service Layer**: Business logic encapsulation
- **Component-Based Architecture**: Modular, reusable components
- **Error Handling**: Comprehensive error management with proper HTTP status codes
- **Type Safety**: Leverages Rust's type system for compile-time guarantees
- **Performance**: Async non-blocking operations with connection pooling

For detailed architecture information, see the [Architecture Documentation](docs/ARCHITECTURE.md).