-- Create users using default password 'Sevria123'
INSERT INTO users
    (id, name, email, password, status)
VALUES
    ('spCe6NvSJ1W2uyybeRIZv', 'admin', 'admin@example.com', '$argon2id$v=19$m=19456,t=2,p=1$ZjIjvMJD0aFgCvD8NHOsOg$abgJlkbliiUacIcIu5VgrwGdXm/Jwu8P1LwIv261suM', 'active');
