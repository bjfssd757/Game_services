CREATE TABLE IF NOT EXISTS user(
    id INTEGER PRIMARY KEY,
    name VARCHAR(100),
    description TEXT
);

CREATE TABLE IF NOT EXISTS product(
    id INTEGER PRIMARY KEY,
    product_name VARCHAR(150),
    count INTEGER
);