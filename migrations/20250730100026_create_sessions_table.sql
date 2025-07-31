CREATE TABLE sessions (
    id CHAR(21) PRIMARY KEY,
    token VARCHAR(100) NOT NULL,
    user_id CHAR(21) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    UNIQUE (token, user_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
