-- Add migration script here
DROP VIEW IF EXISTS tasks;

DROP TABLE IF EXISTS task_chain_links;
DROP TABLE IF EXISTS task_chains;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS task_status;
