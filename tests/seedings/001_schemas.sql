INSERT INTO schemas
    (id, name)
VALUES
    ('Zj9jXWCe5MVaYcJ8Vb-2n', 'posts'),
    ('_EXS0NOjghgyQCg4goPKG', 'comments');

INSERT INTO schema_fields
    (id, schema_id, name, value_type, required, default_value)
VALUES
    ('oNA3ls1H-7iS0RGuCYRAD', 'Zj9jXWCe5MVaYcJ8Vb-2n', 'title', 'string', true, null),
    ('bxF4KfK-h60YmTtrPQFl3', 'Zj9jXWCe5MVaYcJ8Vb-2n', 'content', 'string', true, null),
    ('xMP0e89E30xo_5Kps8Te_', '_EXS0NOjghgyQCg4goPKG', 'content', 'string', true, null);
