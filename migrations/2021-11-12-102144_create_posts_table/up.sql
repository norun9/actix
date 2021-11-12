-- Your SQL goes here
CREATE TABLE posts (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at DATETIME DEFAULT current_timestamp,
    updated_at timestamp default current_timestamp on update current_timestamp
) DEFAULT CHARACTER SET = utf8mb4;