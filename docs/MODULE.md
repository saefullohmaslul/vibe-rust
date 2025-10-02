# Vibe Rust - Module Documentation

This document provides a comprehensive overview of all modules in the Vibe Rust project, a RESTful API for note management built with Rust, Axum, and PostgreSQL.

## Project Architecture

The Vibe Rust project follows a modular architecture with clear separation of concerns and Shaku-based dependency injection:

```
src/
├── main.rs                 # Application entry point + Shaku DI setup
├── infrastructure/         # Infrastructure layer
│   ├── mod.rs             # Infrastructure module declaration
│   └── database.rs        # Database connection and pooling (Shaku component)
├── models/                 # Data models and schemas
│   ├── mod.rs             # Models module declaration
│   └── model.rs           # Database models and response schemas
└── modules/               # Business logic modules
    ├── mod.rs             # Modules module declaration
    ├── commons/           # Common utilities and health checks
    │   ├── mod.rs         # Commons module declaration
    │   └── handler.rs     # Health check handlers + routing
    └── notes/             # Notes management module
        ├── mod.rs         # Notes module declaration + DTOs + Shaku module
        ├── handler.rs     # HTTP request handlers
        ├── service.rs     # Business logic layer (Shaku component)
        └── repository.rs  # Data access layer (Shaku component)
```

## Core Modules

### 1. Main Module (`main.rs`)

**Purpose**: Application entry point and server configuration

**Key Responsibilities**:
- Initialize environment variables using `dotenvy`
- Set up PostgreSQL database connection pool
- Configure CORS (Cross-Origin Resource Sharing)
- Initialize Shaku dependency injection container
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
- `shaku`: Compile-time dependency injection framework

**Key Configuration**:
- Database connection pool with max 5 connections
- CORS allowing GET, POST, and PUT methods from any origin
- Shaku DI container with NotesModule and component parameters
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

### 3. Infrastructure Module (`infrastructure/`)

#### `infrastructure/mod.rs`
**Purpose**: Infrastructure module declaration
- Exports `database` module

#### `infrastructure/database.rs`
**Purpose**: Database connection and pooling management (Shaku component)

**Key Components**:

**`PgPoolComponent`**:
- Shaku component for database connection pool
- Implements dependency injection for database access
- Provides PostgreSQL connection pool to other components

**`PgPoolProvider`** & `PgPoolProviderImpl`**:
- Provider interface and implementation for database access
- Enables loose coupling between components and database
- Manages database connection lifecycle

**Features**:
- Shaku component-based architecture
- Connection pooling with configurable limits
- Type-safe database access patterns
- Centralized database dependency management

### 4. Modules Directory (`modules/`)

#### `modules/mod.rs`
**Purpose**: Module declarations for all business logic modules
- Exports `notes` and `commons` modules

### 5. Commons Module (`modules/commons/`)

#### `commons/mod.rs`
**Purpose**: Commons module declaration
- Exports `handler` module

#### `commons/handler.rs`
**Purpose**: Health check endpoint implementation and routing

**Key Functions**:

**`health()`**:
- Health check endpoint handler
- Returns API status confirmation
- OpenAPI documented with health check response
- Simple JSON response: `{"status": "OK", "message": "API is healthy"}`

**`create_commons_router()`**:
- Creates router for common endpoints
- Registers `/health` GET endpoint
- Returns configured Axum Router

### 6. Notes Module (`modules/notes/`)

#### `notes/mod.rs`
**Purpose**: Notes module coordination, DTO definitions, and Shaku module configuration

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
- Contains: `note_service` (Arc<dyn NoteService>)
- Enables dependency injection with trait objects

**`NotesModule`**:
- Shaku module definition for dependency injection
- Components: [PgPoolComponent, NoteRepositoryImpl, NoteServiceImpl]
- Providers: [PgPoolProviderImpl]
- Manages component lifecycle and dependencies

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
**Purpose**: Business logic layer for notes operations (Shaku component)

**Key Structures**:

**`NoteService`** (trait):
- Business logic interface definition
- Async trait methods for dependency injection
- Defines contract for note operations

**`NoteServiceImpl`**:
- Shaku component implementing NoteService trait
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

**Features**:
- Shaku component-based architecture
- Interface-based programming
- Async trait implementation
- Business logic separation

#### `notes/repository.rs`
**Purpose**: Data access layer for notes database operations (Shaku component)

**Key Structures**:

**`NoteRepository`** (trait):
- Data access interface definition
- Async trait methods for database operations
- Defines contract for data persistence

**`NoteRepositoryImpl`**:
- Shaku component implementing NoteRepository trait
- Database access abstraction
- Uses dependency injection for database pool
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
- Shaku component-based architecture
- Type-safe SQL queries with `sqlx`
- Connection pooling for performance
- Automatic timestamp management
- Interface-based data access

#### `notes/handler.rs` (Routing)
**Purpose**: HTTP request handlers and routing configuration for notes endpoints

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
- **`shaku`**: Compile-time dependency injection framework
- **`async-trait`**: Async trait support for DI interfaces

### API Documentation
- **`utoipa`**: OpenAPI specification generation
- **`utoipa-swagger-ui`**: Interactive API documentation
- Auto-generated API docs with schema validation

### Database
- **PostgreSQL**: Primary database
- **Connection pooling**: Performance optimization via Shaku components
- **Migrations**: Database schema management

### Configuration
- **`dotenvy`**: Environment variable management
- **CORS support**: Cross-origin request handling
- **Dependency Injection**: Shaku-based component management

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
- **Shaku Dependency Injection**: Compile-time DI with trait-based components
- **Repository Pattern**: Data access abstraction with interfaces
- **DTO Pattern**: Request/response models separated from database models
- **Component-Based Architecture**: Modular, reusable components

### Error Handling
- **Consistent Error Responses**: Structured JSON error format
- **Proper HTTP Status Codes**: Semantic use of status codes
- **Graceful Degradation**: Database errors don't crash the server
- **Type Safety**: Compile-time error prevention

### Performance Considerations
- **Connection Pooling**: Database connection reuse via Shaku components
- **Async/Await**: Non-blocking I/O operations
- **Arc Smart Pointers**: Efficient shared state management
- **Pagination**: Prevents large dataset transfers
- **Compile-time DI**: No runtime reflection overhead

## Security Features

- **CORS Configuration**: Controlled cross-origin access
- **SQL Injection Prevention**: Parameterized queries
- **Input Validation**: Type-safe request parsing
- **Environment Variables**: Secure configuration management
- **Interface-based Security**: Clear boundaries between components

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