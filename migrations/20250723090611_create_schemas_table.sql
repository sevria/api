CREATE TABLE schemas (
    id CHAR(21) PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE schema_fields (
    id CHAR(21) PRIMARY KEY,
    schema_id CHAR(21) NOT NULL,
    name VARCHAR(50) NOT NULL,
    value_type VARCHAR(50) NOT NULL,
    required BOOLEAN NOT NULL,
    default_value VARCHAR(100),
    UNIQUE(schema_id, name),
    FOREIGN KEY (schema_id) REFERENCES schemas(id) ON DELETE CASCADE
);
