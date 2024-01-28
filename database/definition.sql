DROP TABLE IF EXISTS vocabulary_list;

CREATE TABLE vocabulary_list (
    id INT NOT NULL AUTO_INCREMENT,
    spelling VARCHAR(255) NOT NULL,
    meaning VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id)
);
