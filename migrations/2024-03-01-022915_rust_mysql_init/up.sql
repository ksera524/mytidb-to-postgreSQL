-- Your SQL goes here

CREATE TABLE stock_prices (
    price_id INT AUTO_INCREMENT PRIMARY KEY,
    stock_symbol VARCHAR(255) NOT NULL,
    market VARCHAR(255) NOT NULL,
    date DATE NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    volume BIGINT NOT NULL
);


