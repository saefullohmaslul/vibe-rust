# Vibe Rust - System Architecture

This document describes the system architecture, design patterns, and technical decisions behind the Vibe Rust REST API application.

## Overview

Vibe Rust is a modern RESTful API built with Rust that demonstrates best practices in web development, database integration, and system design. The architecture follows clean code principles with clear separation of concerns, modular design, and comprehensive error handling.

## System Architecture

### High-Level Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   HTTP Client   │───▶│   Axum Server   │───▶│  PostgreSQL DB  │
│                 │    │                 │    │                 │
│ Swagger UI      │    │ - Routing       │    │ - Notes Table   │
│ REST API Calls  │    │ - Middleware    │    │ - Timestamps    │
│ JSON Requests   │    │ - State Mgmt    │    │ - Triggers      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Layered Architecture

The application follows a classic 3-tier architecture with additional layers for better separation of concerns:

```
┌─────────────────────────────────────────────────────────────┐
│                     Presentation Layer                      │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │   HTTP Routes   │  │   Handlers      │  │  Middleware  │ │
│  │   - REST API    │  │   - Request     │  │  - CORS      │ │
│  │   - Swagger     │  │   - Response    │  │  - Logging   │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      Business Logic Layer                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │   Services      │  │   DTOs          │  │ Validation   │ │
│  │   - Business    │  │   - Request     │  │  - Schema    │ │
│  │   - Rules       │  │   - Response    │  │  - Types     │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                       Data Access Layer                     │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │  Repositories   │  │   Models        │  │  Connection  │ │
│  │   - CRUD Ops    │  │   - Database    │  │  - Pooling   │ │
│  │   - Queries     │  │   - Mapping     │  │  - TX Mgmt   │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      Infrastructure Layer                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │   Database      │  │   Config        │  │  Migration   │ │
│  │   - PostgreSQL  │  │   - Environment │  │  - Version   │ │
│  │   - Docker      │  │   - Secrets     │  │  - Control   │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Design Patterns

### 1. Repository Pattern

**Purpose**: Abstract data access logic and provide a clean interface for database operations.

**Implementation**:
```rust
pub struct NoteRepository {
    pool: Arc<PgPool>,
}

impl NoteRepository {
    pub async fn get_all_notes(&self, limit: i32, offset: i32) -> Result<Vec<NoteModel>, sqlx::Error>
    pub async fn create_note(&self, id: &str, title: &str, content: &str, is_published: bool) -> Result<NoteModel, sqlx::Error>
    pub async fn get_by_id(&self, id: &str) -> Result<NoteModel, sqlx::Error>
    pub async fn update_note(&self, id: &str, title: &str, content: &str, is_published: bool) -> Result<NoteModel, sqlx::Error>
}
```

**Benefits**:
- Separates data access logic from business logic
- Easier unit testing with mock repositories
- Centralized database query management
- Consistent error handling for database operations

### 2. Service Layer Pattern

**Purpose**: Encapsulate business logic and coordinate between repositories and handlers.

**Implementation**:
```rust
pub struct NoteService {
    repository: Arc<NoteRepository>,
}

impl NoteService {
    pub async fn get_notes(&self, opts: FilterOptions) -> Result<Vec<NoteModelResponse>, String>
    pub async fn create_note(&self, note_data: CreateNoteSchema) -> Result<NoteModelResponse, String>
    pub async fn update_note(&self, id: String, note_data: UpdateNoteSchema) -> Result<NoteModelResponse, String>
}
```

**Benefits**:
- Business logic separation from HTTP handling
- Reusable business operations
- Transaction management
- Consistent business rule enforcement

### 3. Shaku Dependency Injection

**Purpose**: Provide compile-time dependency injection with loose coupling and improved testability.

**Implementation**:
```rust
// Module definition
module! {
    pub NotesModule {
        components = [PgPoolComponent, repository::NoteRepositoryImpl, service::NoteServiceImpl],
        providers = [PgPoolProviderImpl]
    }
}

// Component resolution in main.rs
let notes_module = NotesModule::builder()
    .with_component_parameters::<PgPoolComponent>(PgPoolComponentParameters {
        pool: Arc::clone(&pool),
    })
    .build();

let note_service: Arc<dyn NoteService> = notes_module.resolve();
let app_state = Arc::new(AppState { note_service });
```

**Benefits**:
- Compile-time dependency verification
- Easy unit testing with mock dependencies
- Loose coupling between components
- Centralized dependency management
- Interface-based programming
- Automatic component lifetime management

### 4. Data Transfer Object (DTO) Pattern

**Purpose**: Separate API request/response models from database models.

**Implementation**:
```rust
// Request DTOs
#[derive(Deserialize, ToSchema)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    pub is_published: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
}

// Response DTO
#[derive(Serialize, ToSchema)]
pub struct NoteModelResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

**Benefits**:
- API contract stability independent of database schema
- Input validation and transformation
- Selective field exposure
- Version compatibility

### 5. Factory Pattern for Router Configuration

**Purpose**: Centralize route configuration and improve modularity.

**Implementation**:
```rust
pub fn create_notes_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/notes", get(get_list_note_handler))
        .route("/notes", post(create_note_handler))
        .route("/notes/{id}", put(update_note_handler))
        .with_state(app_state)
}

pub fn create_commons_router() -> Router {
    Router::new()
        .route("/health", get(health))
}
```

**Benefits**:
- Modular route configuration
- Easy testing of individual routers
- Clear separation of route concerns
- Simplified main.rs configuration

## Technology Stack Architecture

### Core Framework Selection

**Axum Framework**:
- Built on Tokio for async performance
- Type-safe routing and extraction
- Middleware support
- Excellent ecosystem integration

**Rust Language Benefits**:
- Memory safety without garbage collection
- Zero-cost abstractions
- Fearless concurrency
- Rich type system for compile-time guarantees

### Database Architecture

**PostgreSQL Selection**:
- ACID compliance for data integrity
- Rich feature set (JSONB, arrays, etc.)
- Excellent performance and scalability
- Strong community and tooling support

**SQLx Integration**:
- Compile-time checked queries
- Async-first design
- Connection pooling
- Type-safe database interactions

### Database Schema Design

```sql
CREATE TABLE IF NOT EXISTS notes (
    id CHAR(36) PRIMARY KEY NOT NULL,           -- UUID v4
    title VARCHAR(255) NOT NULL UNIQUE,         -- Unique constraint
    content TEXT NOT NULL,                      -- Long-form content
    is_published BOOLEAN NOT NULL DEFAULT FALSE, -- Publication status
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, -- Creation time
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP  -- Last update
);

-- Automatic timestamp management
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_notes_updated_at
    BEFORE UPDATE ON notes
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Design Decisions**:
- **UUID Primary Keys**: Globally unique, no sequential prediction
- **TIMESTAMPTZ**: Timezone-aware timestamps for global applications
- **Database Triggers**: Automatic timestamp management
- **Unique Title Constraint**: Business rule enforcement at database level

## Request Flow Architecture

### HTTP Request Processing Flow

```
1. HTTP Request
   ↓
2. CORS Middleware
   ↓
3. Router Matching
   ↓
4. State Injection (AppState)
   ↓
5. Handler Function
   ↓
6. Service Layer
   ↓
7. Repository Layer
   ↓
8. Database Operation
   ↓
9. Response Mapping
   ↓
10. HTTP Response
```

### Error Handling Architecture

**Layered Error Handling**:

```rust
// Database Layer (sqlx::Error)
pub async fn get_by_id(&self, id: &str) -> Result<NoteModel, sqlx::Error>

// Service Layer (String error messages)
pub async fn get_notes(&self, opts: FilterOptions) -> Result<Vec<NoteModelResponse>, String>

// Handler Layer (HTTP Status + JSON)
Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
```

**Error Response Format**:
```json
{
  "status": "error",
  "message": "Descriptive error message"
}
```

## Performance Architecture

### Connection Pooling

**Configuration**:
```rust
PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
```

**Benefits**:
- Reduced connection overhead
- Better resource utilization
- Improved throughput
- Connection reuse

### Async Architecture

**Tokio Runtime**:
```rust
#[tokio::main]
async fn main() {
    // Application initialization
}
```

**Benefits**:
- Non-blocking I/O operations
- High concurrency support
- Efficient resource usage
- Better scalability

### Memory Management

**Smart Pointers**:
```rust
Arc<NoteService>    // Shared ownership
Arc<PgPool>         // Shared database pool
Arc<AppState>       // Shared application state
```

**Benefits**:
- Thread-safe sharing
- Automatic memory management
- Reference counting
- No memory leaks

## Security Architecture

### Input Validation

**Schema Validation**:
```rust
#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateNoteSchema {
    pub title: String,        // Required
    pub content: String,      // Required
    pub is_published: Option<bool>,  // Optional
}
```

**Type Safety**:
- Compile-time type checking
- Runtime validation through Serde
- SQL injection prevention via parameterized queries

### CORS Configuration

```rust
let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any)
    .allow_headers([CONTENT_TYPE]);
```

**Security Considerations**:
- Controlled method access
- Configurable origin policies
- Header validation

### Environment Configuration

**Secure Configuration**:
```bash
POSTGRES_DATABASE=rust_axum_sqlx
POSTGRES_USER=postgres
POSTGRES_PASSWORD=12345678
DATABASE_URL=postgresql://...
```

**Best Practices**:
- Environment variable usage
- No hardcoded credentials
- Docker compose for development
- Separate production configurations

## Development Architecture

### Code Organization

```
src/
├── main.rs                 # Application bootstrap + Shaku DI setup
├── infrastructure/         # Infrastructure layer
│   ├── mod.rs             # Module exports
│   └── database.rs        # Database connection and pooling (Shaku component)
├── models/                 # Data models
│   ├── mod.rs             # Module exports
│   └── model.rs           # Database/response models
└── modules/               # Business modules
    ├── mod.rs             # Module registry
    ├── commons/           # Shared utilities
    │   ├── mod.rs
    │   └── handler.rs     # Health checks + common routes
    └── notes/             # Notes module
        ├── mod.rs         # Module exports + DTOs + AppState + Shaku module
        ├── handler.rs     # HTTP handlers
        ├── service.rs     # Business logic (Shaku component)
        └── repository.rs  # Data access (Shaku component)
```

### Shaku Dependency Injection Architecture

**Module Structure**:
```rust
module! {
    pub NotesModule {
        components = [PgPoolComponent, repository::NoteRepositoryImpl, service::NoteServiceImpl],
        providers = [PgPoolProviderImpl]
    }
}
```

**Component Registration**:
- `PgPoolComponent`: Database connection pool provider
- `NoteRepositoryImpl`: Repository implementation
- `NoteServiceImpl`: Service implementation
- `PgPoolProviderImpl`: Provider interface for database access

**DI Benefits**:
- Loose coupling between components
- Easy unit testing with mock dependencies
- Centralized dependency management
- Compile-time dependency verification

### Migration Architecture

**Version Control**:
```bash
migrations/
├── 20251002034419_create_notes_table.up.sql
├── 20251002034419_create_notes_table.down.sql
├── 20251002080733_fix_timestamp_types.up.sql
└── 20251002080733_fix_timestamp_types.down.sql
```

**Migration Management**:
```makefile
migration:create:   # Create new migration
migration:run:      # Apply pending migrations
migration:revert:   # Rollback last migration
```

### Documentation Architecture

**OpenAPI Integration**:
```rust
#[derive(OpenApi)]
#[openapi(
    paths(...),
    components(schemas(...)),
    info(title = "Vibe Rust API", ...)
)]
struct ApiDoc;
```

**Auto-Generated Documentation**:
- `/swagger-ui` - Interactive API documentation
- `/api-docs/openapi.json` - OpenAPI specification
- Schema validation and testing

## Scalability Architecture

### Horizontal Scaling Readiness

**Stateless Design**:
- No in-memory state between requests
- External database for state persistence
- Load balancer friendly

**Database Scaling**:
- Connection pooling support
- Read replica potential
- Sharding capabilities with PostgreSQL

### Monitoring and Observability

**Current Implementation**:
- Structured logging through error responses
- Health check endpoint for monitoring
- Database connection metrics

**Future Enhancements**:
- Request tracing
- Performance metrics
- Error rate monitoring
- Custom dashboards

## Deployment Architecture

### Container Strategy

**Docker Compose**:
```yaml
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: ${POSTGRES_DATABASE}
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
```

**Benefits**:
- Development environment consistency
- Easy database setup
- Volume persistence
- Environment isolation

### Environment Management

**Development**:
- Docker Compose for local development
- Hot reload with `cargo watch`
- Environment variables for configuration

**Production Considerations**:
- Environment-specific configurations
- Secret management
- Database backups
- SSL/TLS termination

## Future Architecture Evolution

### Potential Enhancements

1. **Caching Layer**: Redis for frequently accessed data
2. **Message Queue**: Async processing for background tasks
3. **API Gateway**: Request routing and rate limiting
4. **Microservices**: Module separation into independent services
5. **Event Sourcing**: Audit trail and event-driven architecture

### Scaling Strategies

1. **Database Optimization**: Query optimization and indexing
2. **Caching Strategy**: Multi-level caching implementation
3. **Load Balancing**: Multiple application instances
4. **CDN Integration**: Static asset delivery
5. **Monitoring**: Comprehensive observability stack

## Architectural Decision Records (ADRs)

### ADR-001: Choice of Axum Framework
**Decision**: Use Axum over alternatives like Actix-web or Rocket
**Rationale**: Better Tokio integration, type-safe routing, modern async design

### ADR-002: PostgreSQL Database Selection
**Decision**: Use PostgreSQL instead of SQLite or other databases
**Rationale**: Production-ready, feature-rich, excellent Rust support via SQLx

### ADR-003: UUID Primary Keys
**Decision**: Use UUID v4 instead of sequential integers
**Rationale**: Global uniqueness, no prediction, better distributed systems support

### ADR-004: Repository Pattern Implementation
**Decision**: Implement repository pattern for data access
**Rationale**: Testability, separation of concerns, easier maintenance

### ADR-005: OpenAPI Integration
**Decision**: Use utoipa for automatic API documentation
**Rationale**: Type-safe documentation generation, always in sync with code

This architecture provides a solid foundation for a scalable, maintainable, and performant REST API while following Rust best practices and modern web development patterns.