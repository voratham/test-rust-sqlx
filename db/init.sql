CREATE TABLE product (
    product_id INT PRIMARY KEY AUTO_INCREMENT,
    product_name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    quantity INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=INNODB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8;

INSERT INTO product (product_name, price, quantity, created_at, updated_at) VALUES
('Product A', 19.99, 100, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('Product B', 29.99, 200, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('Product C', 39.99, 150, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('Product D', 49.99, 50, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('Product E', 59.99, 75, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);