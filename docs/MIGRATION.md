# Database Migrations

This document explains how to manage database migrations using `sqlx-cli` in this project.

## Prerequisites

- PostgreSQL database must be running
- Database URL should be set in your environment variables (e.g., `DATABASE_URL`)
- `sqlx-cli` must be installed: `cargo install sqlx-cli --no-default-features --features native-tls,postgres`

## Creating New Migrations

To create a new migration, use the following command:

```bash
sqlx migrate add -r <migration_name>
```

The `-r` flag creates both up and down migration files.

### Example

To create a notes table migration:

```bash
sqlx migrate add -r create_notes_table
```

This will create two files in the `migrations/` directory:
- `xxxxxxxxxxxxxx_create_notes_table.up.sql` - for applying the migration
- `xxxxxxxxxxxxxx_create_notes_table.down.sql` - for rolling back the migration

## Migration File Structure

### Up Migration (.up.sql)
Contains the SQL statements to apply the migration:

```sql
CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Down Migration (.down.sql)
Contains the SQL statements to rollback the migration:

```sql
DROP TABLE IF EXISTS notes;
```

## Running Migrations

### Apply All Pending Migrations
```bash
sqlx migrate run
```

### Apply Migrations to a Specific Version
```bash
sqlx migrate run --version <version_number>
```

## Managing Migrations

### Check Migration Status
```bash
sqlx migrate info
```

### Rollback Last Migration
```bash
sqlx migrate revert
```

### Rollback to Specific Version
```bash
sqlx migrate revert --version <version_number>
```

### Build Migration Script
```bash
sqlx migrate build-script --target-dir <output_directory>
```

## Best Practices

1. **Descriptive Names**: Use clear, descriptive names for migrations that describe what they do
2. **Reversible Migrations**: Always provide both up and down migrations when possible
3. **Test Migrations**: Test your migrations on a development database before applying to production
4. **Backup Data**: Always backup your database before running migrations in production
5. **Idempotent**: Design migrations that can be safely re-run if needed

## Common Migration Patterns

### Creating Tables
```sql
-- up.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- down.sql
DROP TABLE IF EXISTS users;
```

### Adding Columns
```sql
-- up.sql
ALTER TABLE users ADD COLUMN last_login TIMESTAMP WITH TIME ZONE;

-- down.sql
ALTER TABLE users DROP COLUMN IF EXISTS last_login;
```

### Adding Indexes
```sql
-- up.sql
CREATE INDEX idx_users_email ON users(email);

-- down.sql
DROP INDEX IF EXISTS idx_users_email;
```

## Troubleshooting

### Migration Fails
If a migration fails:
1. Check the SQL syntax
2. Verify database connectivity
3. Ensure you have necessary permissions
4. Check if the migration creates conflicts with existing data

### Migration Gets Stuck
If a migration appears to be stuck:
1. Check the current migration status with `sqlx migrate info`
2. Verify the database connection
3. Check PostgreSQL logs for any errors
4. Manually rollback if necessary using `sqlx migrate revert`

## Environment Variables

Set these environment variables before running migrations:

```bash
export DATABASE_URL="postgresql://username:password@localhost/database_name"
```

For development, you can also use a `.env` file in the project root.