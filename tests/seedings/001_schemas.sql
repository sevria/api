INSERT INTO schemas
    (id, name)
VALUES
    (1, 'posts'),
    (2, 'comments'),
    (3, 'old_posts');

SELECT setval('schemas_id_seq', (SELECT MAX(id) FROM schemas));
