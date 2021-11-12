---- drop ----
DROP TABLE IF EXISTS `posts`;
---- create ----
create table IF not exists `posts` (
    `id` INT(20) AUTO_INCREMENT,
    `body` VARCHAR(20) NOT NULL,
    `created_at` Datetime DEFAULT NULL,
    `updated_at` Datetime DEFAULT NULL,
    PRIMARY KEY (`id`)
) DEFAULT CHARSET = utf8 COLLATE = utf8_bin;