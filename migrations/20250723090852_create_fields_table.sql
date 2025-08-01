CREATE TABLE sevria_fields (
    schema_id CHAR(21) NOT NULL,
    name VARCHAR(50) NOT NULL,
    value_type VARCHAR(50) NOT NULL,
    required BOOLEAN NOT NULL,
    default_value VARCHAR(100),
    PRIMARY KEY (schema_id, name),
    FOREIGN KEY (schema_id) REFERENCES sevria_schemas(id) ON DELETE CASCADE
);
