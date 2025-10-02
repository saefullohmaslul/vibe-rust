-- Revert timestamp types back to TIMESTAMP
ALTER TABLE notes
ALTER COLUMN created_at TYPE TIMESTAMP,
ALTER COLUMN updated_at TYPE TIMESTAMP;