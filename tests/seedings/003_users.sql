INSERT INTO users
    (id, name, email, password, status)
VALUES
    (1, 'admin', 'admin@example.com', 'ARGON2ID-HASH', 'active');

SELECT setval('users_id_seq', (SELECT MAX(id) FROM users));
