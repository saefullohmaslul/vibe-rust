# Vibe Rust - Module Documentation

This document provides a comprehensive overview of all modules in the Vibe Rust project, a RESTful API for note management built with Rust, Axum, and PostgreSQL.

## Project Architecture

The Vibe Rust project follows a modular architecture with clear separation of concerns:

```
src/
├── main.rs                 # Application entry point
├── models/                 # Data models and schemas
│   ├── mod.rs             # Models module declaration
│   └── model.rs           # Database models and response schemas
└── modules/               # Business logic modules
    ├── mod.rs             # Modules module declaration
    ├── commons/           # Common utilities and health checks
    │   ├── mod.rs         # Commons module declaration
    │   ├── handler.rs     # Health check handlers
    │   └── routes.rs      # Commons routing configuration
    └── notes/             # Notes management module
        ├── mod.rs         # Notes module declaration and schemas
        ├── handler.rs     # HTTP request handlers
        ├── service.rs     # Business logic layer
        ├── repository.rs  # Data access layer
        └── routes.rs      # Notes routing configuration
```

## Core Modules

### 1. Main Module (`main.rs`)

**Purpose**: Application entry point and server configuration

**Key Responsibilities**:
- Initialize environment variables using `dotenvy`
- Set up PostgreSQL database connection pool
- Configure CORS (Cross-Origin Resource Sharing)
- Create and configure the Axum router
- Set up Swagger UI for API documentation
- Start the HTTP server on port 8080

**Dependencies**:
- `axum`: Web framework for HTTP routing and middleware
- `sqlx`: Database toolkit for PostgreSQL
- `tokio`: Async runtime
- `tower-http`: HTTP middleware (CORS)
- `utoipa`: OpenAPI documentation generation
- `uuid`: UUID generation for note IDs

**Key Configuration**:
- Database connection pool with max 5 connections
- CORS allowing GET and POST methods from any origin
- API routes mounted under `/api/v1`
- Swagger UI available at `/swagger-ui`

### 2. Models Module (`models/`)

#### `models/mod.rs`
**Purpose**: Module declaration for models

#### `models/model.rs`
**Purpose**: Database models and API response schemas

**Key Structures**:

**`NoteModel`**:
- Database entity representation
- Fields: `id`, `title`, `content`, `is_published`, `created_at`, `updated_at`
- Implements `FromRow` for SQL mapping
- Uses `chrono::DateTime` for timestamps

**`NoteModelResponse`**:
- API response model for notes
- Identical structure to `NoteModel` but separated for API contract stability
- Used for consistent API responses

**Features**:
- Serialization/deserialization with `serde`
- OpenAPI schema generation with `utoipa::ToSchema`
- Automatic timestamp handling with `chrono`

### 3. Modules Directory (`modules/`)

#### `modules/mod.rs`
**Purpose**: Module declarations for all business logic modules
- Exports `notes` and `commons` modules

### 4. Commons Module (`modules/commons/`)

#### `commons/mod.rs`
**Purpose**: Commons module declaration
- Exports `handler` and `routes` modules

#### `commons/handler.rs`
**Purpose**: Health check endpoint implementation

**Key Functions**:

**`health()`**:
- Health check endpoint handler
- Returns API status confirmation
- OpenAPI documented with health check response
- Simple JSON response: `{"status": "OK", "message": "API is healthy"}`

#### `commons/routes.rs`
**Purpose**: Commons routing configuration

**Key Functions**:

**`create_commons_router()`**:
- Creates router for common endpoints
- Registers `/health` GET endpoint
- Returns configured Axum Router

### 5. Notes Module (`modules/notes/`)

#### `notes/mod.rs`
**Purpose**: Notes module coordination and DTO definitions

**Key Structures**:

**`FilterOptions`**:
- Query parameters for note listing
- Fields: `page` (Optional<usize>), `limit` (Optional<usize>)
- Default values: page=1, limit=10

**`CreateNoteSchema`**:
- Request DTO for note creation
- Fields: `title` (String), `content` (String), `is_published` (Optional<bool>)
- Validation through `serde` deserialization

**`UpdateNoteSchema`**:
- Request DTO for note updates
- All fields are optional for partial updates
- Fields: `title` (Option<String>), `content` (Option<String>), `is_published` (Option<bool>)

**`AppState`**:
- Application state shared across handlers
- Contains: `note_service` (Arc<NoteService>)
- Enables dependency injection

#### `notes/handler.rs`
**Purpose**: HTTP request handlers for notes API

**Key Functions**:

**`get_list_note_handler()`**:
- GET `/api/v1/notes` endpoint
- Supports pagination via query parameters
- Returns paginated list of notes
- Error handling with proper HTTP status codes

**`create_note_handler()`**:
- POST `/api/v1/notes` endpoint
- Creates new notes with auto-generated UUID
- Validates request body against `CreateNoteSchema`
- Returns created note with generated metadata

**`update_note_handler()`**:
- PUT `/api/v1/notes/{id}` endpoint
- Updates existing notes by ID
- Validates UUID format in path parameter
- Supports partial updates
- Returns updated note with new timestamp

**Features**:
- Comprehensive OpenAPI documentation for all endpoints
- Consistent error handling with structured JSON responses
- Request validation and type safety
- Proper HTTP status codes

#### `notes/service.rs`
**Purpose**: Business logic layer for notes operations

**Key Structure**:

**`NoteService`**:
- Business logic orchestrator
- Depends on `NoteRepository` for data access
- Handles UUID generation and validation
- Manages data transformation between models

**Key Methods**:

**`get_notes(opts: FilterOptions)`**:
- Retrieves paginated notes
- Applies pagination logic (limit/offset)
- Transforms database models to response models

**`create_note(note_data: CreateNoteSchema)`**:
- Creates new notes with UUID generation
- Sets default values (published=false)
- Transforms and returns created note

**`update_note(id: String, note_data: UpdateNoteSchema)`**:
- Updates existing notes
- Validates UUID format
- Merges partial updates with existing data
- Handles not found scenarios

**`to_note_response(note: &NoteModel)`**:
- Internal helper for model transformation
- Ensures consistent API response format

#### `notes/repository.rs`
**Purpose**: Data access layer for notes database operations

**Key Structure**:

**`NoteRepository`**:
- Database access abstraction
- Uses connection pooling via `Arc<PgPool>`
- Direct SQL query execution

**Key Methods**:

**`get_all_notes(limit: i32, offset: i32)`**:
- Retrieves paginated notes from database
- Uses parameterized SQL queries
- Returns vector of `NoteModel`

**`create_note(id, title, content, is_published)`**:
- Inserts new note into database
- Uses PostgreSQL `RETURNING` clause
- Auto-generates timestamps via database

**`get_by_id(id)`**:
- Retrieves single note by UUID
- Returns single `NoteModel` or error

**`update_note(id, title, content, is_published)`**:
- Updates existing note
- Automatically updates `updated_at` timestamp
- Returns updated note

**Features**:
- Type-safe SQL queries with `sqlx`
- Connection pooling for performance
- Automatic timestamp management
- Error handling for database operations

#### `notes/routes.rs`
**Purpose**: Routing configuration for notes endpoints

**Key Functions**:

**`create_notes_router(app_state: Arc<AppState>)`**:
- Configures all notes-related routes
- Maps HTTP methods to handlers:
  - GET `/notes` → `get_list_note_handler`
  - POST `/notes` → `create_note_handler`
  - PUT `/notes/{id}` → `update_note_handler`
- Injects application state for dependency injection

## Technology Stack

### Core Dependencies
- **`axum`**: Modern, ergonomic web framework built on Tokio
- **`sqlx`**: Async SQL toolkit with compile-time verification
- **`tokio`**: Asynchronous runtime for Rust
- **`serde`**: Serialization/deserialization framework
- **`uuid`**: UUID generation and parsing
- **`chrono`**: Date and time handling

### API Documentation
- **`utoipa`**: OpenAPI specification generation
- **`utoipa-swagger-ui`**: Interactive API documentation
- Auto-generated API docs with schema validation

### Database
- **PostgreSQL**: Primary database
- **Connection pooling**: Performance optimization
- **Migrations**: Database schema management

### Configuration
- **`dotenvy`**: Environment variable management
- **CORS support**: Cross-origin request handling
- **Structured logging**: Error tracking and monitoring

## API Endpoints

### Commons
- `GET /api/v1/health` - Health check endpoint

### Notes
- `GET /api/v1/notes` - List notes with pagination
- `POST /api/v1/notes` - Create new note
- `PUT /api/v1/notes/{id}` - Update existing note

### Documentation
- `/swagger-ui` - Interactive API documentation
- `/api-docs/openapi.json` - OpenAPI specification

## Design Patterns

### Architecture Patterns
- **Layered Architecture**: Clear separation between handlers, services, and repositories
- **Dependency Injection**: Service dependencies injected via `AppState`
- **Repository Pattern**: Data access abstraction
- **DTO Pattern**: Request/response models separated from database models

### Error Handling
- **Consistent Error Responses**: Structured JSON error format
- **Proper HTTP Status Codes**: Semantic use of status codes
- **Graceful Degradation**: Database errors don't crash the server

### Performance Considerations
- **Connection Pooling**: Database connection reuse
- **Async/Await**: Non-blocking I/O operations
- **Arc Smart Pointers**: Efficient shared state management
- **Pagination**: Prevents large dataset transfers

## Security Features

- **CORS Configuration**: Controlled cross-origin access
- **SQL Injection Prevention**: Parameterized queries
- **Input Validation**: Type-safe request parsing
- **Environment Variables**: Secure configuration management

## Development Workflow

### Running the Application
```bash
cargo run
```

### API Testing
- Swagger UI available at `http://localhost:8080/swagger-ui`
- Health check at `http://localhost:8080/api/v1/health`

### Database
- PostgreSQL with connection pooling
- Schema migrations managed via `migrations/` directory
- Auto-generated timestamps for audit trails

This modular architecture ensures maintainability, testability, and scalability while following Rust best practices and modern web development patterns.