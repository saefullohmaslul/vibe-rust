.PHONY: migration\:create migration\:run migration\:revert

help:
	@echo "Usage:"
	@echo "  make migration:create NAME=migration_name"
	@echo "  make migration:run"
	@echo "  make migration:revert"

# Create new migration
migration\:create:
	@if [ -z "$(NAME)" ]; then \
		echo "Usage: make migration:create NAME=migration_name"; \
		exit 1; \
	fi
	sqlx migrate add $(NAME)

# Run all pending migrations
migration\:run:
	sqlx migrate run

# Revert last migration
migration\:revert:
	sqlx migrate revert