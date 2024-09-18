DROP TABLE IF EXISTS "user";
CREATE TABLE "user" (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

DROP TABLE IF EXISTS trading_bot;
CREATE TABLE trading_bot (
    id BIGSERIAL PRIMARY KEY,
    rule VARCHAR(255)
);

DROP TABLE IF EXISTS wallet;
CREATE TABLE wallet (
    id BIGSERIAL PRIMARY KEY,
    address VARCHAR(255) NOT NULL,
    user_id BIGINT NOT NULL
);

DROP TABLE IF EXISTS product;
CREATE TABLE product (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL
);

DROP TABLE IF EXISTS item;
CREATE TABLE item (
    id BIGSERIAL PRIMARY KEY,
    product_id BIGINT NOT NULL,
    quantity INT NOT NULL,
    FOREIGN KEY (product_id) REFERENCES product(id)
);
