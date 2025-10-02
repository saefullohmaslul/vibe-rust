# Dependencies

This document explains the purpose and usage of each dependency in the vibe-rust project.

## Core Framework & Runtime

### axum (0.8.6)
A modern web framework built on top of Tokio for building HTTP servers and REST APIs. Axum provides a type-safe and ergonomic API for handling HTTP requests, routing, middleware, and extracting data from requests.

### tokio (1.47.1)
The asynchronous runtime for Rust. Essential for running async applications in Rust. This crate provides the necessary runtime, async I/O, timers, and utilities for building concurrent applications.

## Database & Persistence

### sqlx (0.8.6)
An async SQL toolkit with compile-time checked queries. This crate provides:
- Database connection pooling
- Async query execution
- Type-safe query building
- Support for PostgreSQL with additional features for chrono and UUID integration

### chrono (0.4.42)
Date and time manipulation library. Provides comprehensive types and functions for working with dates, times, and durations in Rust applications. The serde feature enables easy serialization/deserialization of datetime values.

## Serialization & Data Handling

### serde (1.0.228)
The serialization and deserialization framework for Rust. Provides the derive macros to automatically implement serialization traits for custom structs and enums.

### serde_json (1.0.145)
JSON serialization and deserialization implementation for serde. Used for converting Rust data structures to and from JSON format for API responses and requests.

### uuid (1.18.1)
Universally Unique Identifier (UUID) generation and parsing library. The v4 feature enables generation of random UUIDs, while serde feature allows for easy serialization/deserialization in API contexts.

## Configuration & Utilities

### dotenvy (0.15.7)
Environment variable loader. Reads configuration from `.env` files during development, making it easy to manage environment-specific settings like database URLs, API keys, and other configuration values.

### tower-http (0.6.6)
HTTP-specific middleware for Tower ecosystem. Provides commonly needed HTTP middleware including CORS (Cross-Origin Resource Sharing) handling for web APIs that need to serve clients from different origins.

## Dependency Injection

### shaku (0.6)
A compile-time dependency injection framework for Rust. Provides:
- Component-based architecture with interfaces and implementations
- Compile-time dependency verification
- Module system for organizing components
- Provider pattern for dependency resolution

### async-trait (0.1)
Provides support for async functions in traits. Essential for defining async interfaces that can be implemented by different components in the dependency injection system.

## API Documentation

### utoipa (5.4.0)
OpenAPI specification generation library with Rust derive macros. Features:
- Automatic OpenAPI schema generation from Rust structs
- Support for Axum framework integration
- Chrono datetime serialization support
- Type-safe API documentation

### utoipa-swagger-ui (9.0.2)
Swagger UI integration for serving interactive API documentation. Provides:
- Web-based API explorer
- Real-time API testing
- Auto-generated documentation from OpenAPI specs
- Axum framework integration

## Infrastructure

### PostgreSQL (16)
Relational database management system used as the primary data store for the application. PostgreSQL provides robust SQL capabilities, ACID compliance, and excellent performance for both OLTP and analytical workloads. The application uses PostgreSQL 16 running in a Docker container for consistent development and deployment environments.

#### Configuration (docker-compose.yml)
- **Container**: `rust_postgres`
- **Image**: `postgres:16`
- **Port**: `5432:5432` (host:container)
- **Volume**: `postgres_data:/var/lib/postgresql/data` for persistent storage
- **Restart Policy**: `unless-stopped`

#### Environment Variables (.env)
- **Database**: `rust_axum_sqlx`
- **User**: `postgres`
- **Connection URL**: `postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:5432/${POSTGRES_DATABASE}`

## Usage Summary

These dependencies form a complete stack for building modern web APIs in Rust:
- **axum** provides the web server framework
- **tokio** powers the async runtime
- **sqlx** handles database operations with PostgreSQL
- **serde** family manages data serialization
- **dotenvy** handles configuration including database credentials
- **tower-http** provides HTTP middleware like CORS
- **shaku** provides compile-time dependency injection
- **async-trait** enables async trait methods
- **utoipa** generates API documentation automatically
- **postgresql** serves as the relational database backend