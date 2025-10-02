.PHONY: help build run dev test test\:watch test\:verbose clean lint fmt fmt-check check migration\:create migration\:run migration\:revert migration\:info db\:up db\:down db\:logs db\:shell docker\:build docker\:run docker\:stop docs\:open install-deps setup reset-db health start stop

# Default target
help:
	@echo "Vibe Rust - Available Commands:"
	@echo ""
	@echo "Development:"
	@echo "  make build          Build the project"
	@echo "  make run            Run the application"
	@echo "  make dev            Run in development mode with hot reload"
	@echo "  make test           Run tests"
	@echo "  make test:watch     Run tests in watch mode"
	@echo "  make test:verbose   Run tests with verbose output"
	@echo "  make clean          Clean build artifacts"
	@echo "  make lint           Run clippy linter"
	@echo "  make fmt            Format code"
	@echo "  make check          Run all checks (fmt, clippy, test)"
	@echo ""
	@echo "Database:"
	@echo "  make db:up          Start PostgreSQL container"
	@echo "  make db:down        Stop PostgreSQL container"
	@echo "  make db:logs        Show database logs"
	@echo "  make db:shell       Access database shell"
	@echo "  make migration:create NAME=migration_name  Create new migration"
	@echo "  make migration:run  Run all pending migrations"
	@echo "  make migration:revert    Revert last migration"
	@echo "  make migration:info Show migration status"
	@echo ""
	@echo "Docker:"
	@echo "  make docker:build   Build Docker image"
	@echo "  make docker:run     Run Docker container"
	@echo "  make docker:stop    Stop Docker container"
	@echo ""
	@echo "Documentation:"
	@echo "  make docs:open      Open API documentation in browser"
	@echo ""
	@echo "Examples:"
	@echo "  make migration:create NAME=add_user_table"
	@echo "  make db:up && make migration:run && make run"

# Development commands
build:
	@echo "🔨 Building project..."
	cargo build

build-release:
	@echo "🔨 Building project in release mode..."
	cargo build --release

run:
	@echo "🚀 Starting application..."
	cargo run

dev:
	@echo "🔥 Starting development server with hot reload..."
	cargo watch -q -c -w src/ -x run

test:
	@echo "🧪 Running tests..."
	cargo test

test\:watch:
	@echo "🧪 Running tests in watch mode..."
	cargo watch -x test

test\:verbose:
	@echo "🧪 Running tests with verbose output..."
	cargo test -- --nocapture

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

lint:
	@echo "🔍 Running clippy linter..."
	cargo clippy -- -D warnings

fmt:
	@echo "✨ Formatting code..."
	cargo fmt

fmt-check:
	@echo "✨ Checking code formatting..."
	cargo fmt --check

check: fmt-check lint test
	@echo "✅ All checks completed!"

# Database commands
db\:up:
	@echo "🐘 Starting PostgreSQL container..."
	docker-compose up -d postgres

db\:down:
	@echo "🛑 Stopping PostgreSQL container..."
	docker-compose down postgres

db\:logs:
	@echo "📋 Showing database logs..."
	docker-compose logs -f postgres

db\:shell:
	@echo "🐚 Opening database shell..."
	docker-compose exec postgres psql -U postgres -d rust_axum_sqlx

# Migration commands
migration\:create:
	@if [ -z "$(NAME)" ]; then \
		echo "❌ Usage: make migration:create NAME=migration_name"; \
		exit 1; \
	fi
	@echo "📝 Creating migration: $(NAME)"
	sqlx migrate add $(NAME)

migration\:run:
	@echo "⬆️ Running database migrations..."
	sqlx migrate run

migration\:revert:
	@echo "⬇️ Reverting last migration..."
	sqlx migrate revert

migration\:info:
	@echo "ℹ️ Migration information:"
	sqlx migrate info

# Docker commands
docker\:build:
	@echo "🐳 Building Docker image..."
	docker build -t vibe-rust .

docker\:run:
	@echo "🐳 Running Docker container..."
	docker run -p 8080:8080 --env-file .env vibe-rust

docker\:stop:
	@echo "🛑 Stopping Docker containers..."
	docker-compose down

# Documentation commands
docs\:open:
	@echo "📖 Opening API documentation in browser..."
	@echo "📗 Swagger UI: http://localhost:8080/swagger-ui"
	@echo "📘 OpenAPI JSON: http://localhost:8080/api-docs/openapi.json"
	@if command -v open >/dev/null 2>&1; then \
		sleep 2 && open http://localhost:8080/swagger-ui; \
	elif command -v xdg-open >/dev/null 2>&1; then \
		sleep 2 && xdg-open http://localhost:8080/swagger-ui; \
	else \
		echo "Please open http://localhost:8080/swagger-ui in your browser"; \
	fi

# Utility commands
install-deps:
	@echo "📦 Installing required tools..."
	cargo install cargo-watch
	cargo install sqlx-cli --no-default-features --features native-tls,postgres

setup:
	@echo "🔧 Setting up development environment..."
	@if [ ! -f .env ]; then \
		cp .env.example .env; \
		echo "✅ Created .env file from .env.example"; \
	else \
		echo "✅ .env file already exists"; \
	fi
	@echo "🐘 Starting database..."
	make db:up
	@echo "⏳ Waiting for database to be ready..."
	sleep 5
	@echo "⬆️ Running migrations..."
	make migration:run
	@echo "✅ Setup complete! Run 'make run' to start the application."

reset-db:
	@echo "🔄 Resetting database..."
	@echo "⚠️  This will delete all data!"
	@read -p "Are you sure? (y/N): " confirm && [ "$$confirm" = "y" ] || exit 1
	docker-compose down -v
	docker-compose up -d postgres
	sleep 5
	make migration:run
	@echo "✅ Database reset complete!"

# Health check
health:
	@echo "🏥 Checking application health..."
	@curl -s http://localhost:8080/api/v1/health | jq '.' || echo "❌ Application not running or health check failed"

# Quick start (combines common commands)
start: setup run
	@echo "🚀 Application is running at http://localhost:8080"

stop:
	@echo "🛑 Stopping all services..."
	docker-compose down
	pkill -f "cargo run" || true
	@echo "✅ All services stopped"