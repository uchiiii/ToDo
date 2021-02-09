-- Your SQL goes here

CREATE TABLE `todos` (
    id INTEGER unsigned NOT NULL AUTO_INCREMENT,
    text TEXT COLLATE utf8mb4_unicode_ci NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;