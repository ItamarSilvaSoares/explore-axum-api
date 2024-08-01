CREATE DATABASE IF NOT EXISTS explore_axum_db;

USE explore_axum_db;

CREATE TABLE IF not EXISTS users (
    id int primary key auto_increment,
    name varchar(200) not null,
    email varchar(200) not null
);

INSERT INTO users (id, name, email)
(SELECT 1, 'Alice Smith', 'alice.smith@example.com') UNION ALL
(SELECT 2, 'Bob Johnson', 'bob.johnson@example.com') UNION ALL
(SELECT 3, 'Charlie Lee', 'charlie.lee@example.com') UNION ALL
(SELECT 4, 'Dana White', 'dana.white@example.com') UNION ALL
(SELECT 5, 'Evan Brown', 'evan.brown@example.com'
WHERE NOT EXISTS (SELECT * FROM users));