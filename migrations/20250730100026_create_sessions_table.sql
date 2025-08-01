CREATE TABLE sevria_sessions (
    id CHAR(21) PRIMARY KEY,
    token VARCHAR(36) NOT NULL,
    user_id CHAR(21) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    UNIQUE (token, user_id),
    FOREIGN KEY (user_id) REFERENCES sevria_users(id) ON DELETE CASCADE
);
