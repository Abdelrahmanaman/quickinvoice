-- Your SQL goes here
-- migrations/YYYY-MM-DD-HHMMSS_create_webhooks/up.sql
CREATE TABLE webhooks (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL
);