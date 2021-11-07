-- Your SQL goes here
CREATE TYPE category AS ENUM('flower', 'pre_roll', 'edible', 'cartridge', 'extract', 'accessory', 'other');
CREATE TYPE family AS ENUM('indica', 'sativa', 'hybrid');

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    category CATEGORY NOT NULL,
    UNIQUE (name, category)
);

CREATE TABLE cannabis (
    id SERIAL PRIMARY KEY,
    product_id INT NOT NULL,
    family FAMILY NOT NULL,
    thc FLOAT4 NOT NULL,
    cbd FLOAT4 NOT NULL,
    total_cannabinoids FLOAT4 NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE CASCADE
);

CREATE TABLE inventories (
    id SERIAL PRIMARY KEY,
    product_id INT NOT NULL,
    stock INT NOT NULL,
    price FLOAT4 NOT NULL,
    net_weight FLOAT4 NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products (id) ON DELETE CASCADE
);

CREATE TABLE terpenes (
    id SERIAL PRIMARY KEY,
    cannabis_id INT NOT NULL,
    myrcene FLOAT4 NOT NULL DEFAULT 0.0,
    pinene FLOAT4 NOT NULL DEFAULT 0.0,
    limonene FLOAT4 NOT NULL DEFAULT 0.0,
    caryophyllene FLOAT4 NOT NULL DEFAULT 0.0,
    terpinolene FLOAT4 NOT NULL DEFAULT 0.0,
    FOREIGN KEY (cannabis_id) REFERENCES cannabis (id) ON DELETE CASCADE
);

CREATE TABLE batches (
    id SERIAL PRIMARY KEY,
    cannabis_id INT NOT NULL,
    harvest_date DATE NOT NULL,
    package_date DATE NOT NULL,
    final_test_date DATE NOT NULL,
    FOREIGN KEY (cannabis_id) REFERENCES cannabis (id) ON DELETE CASCADE
);
