DROP TABLE IF EXISTS "user";
CREATE TABLE "user" (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255),
    username VARCHAR(255),
    password VARCHAR(255)
);

DROP TABLE IF EXISTS trading_bot;
CREATE TABLE trading_bot (
    id BIGSERIAL PRIMARY KEY,
    rule VARCHAR(255)
);
