-- Your SQL goes here
CREATE TABLE memos (
    `id` INT UNSIGNED AUTO_INCREMENT,
    `title` VARCHAR(255) NOT NULL,
    `body` TEXT NOT NULL,
    `published` BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
