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

## Usage Summary

These dependencies form a complete stack for building modern web APIs in Rust:
- **axum** provides the web server framework
- **tokio** powers the async runtime
- **sqlx** handles database operations
- **serde** family manages data serialization
- **dotenvy** handles configuration
- **tower-http** provides HTTP middleware like CORS