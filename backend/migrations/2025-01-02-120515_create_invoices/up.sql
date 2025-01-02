-- Your SQL goes here
CREATE TABLE invoices (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount NUMERIC NOT NULL,
    currency TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
);