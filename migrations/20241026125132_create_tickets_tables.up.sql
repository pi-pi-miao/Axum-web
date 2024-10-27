-- Add up migration script here
CREATE TABLE IF NOT EXISTS `tickets` (
                                         `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
                                         `assignee_id` BIGINT UNSIGNED NULL,
                                         `title` VARCHAR(255) NOT NULL,
                                         `description` VARCHAR(255) NULL,
                                         `body` TEXT NULL,
                                         `status` TINYINT(1) NOT NULL DEFAULT 0,
                                         `created_at` TIMESTAMP NULL,
                                         `updated_at` TIMESTAMP NULL,
                                         PRIMARY KEY (`id`)
);
