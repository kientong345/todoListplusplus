-- Add down migration script here
DROP TRIGGER update_usr_updated_at ON users;
DROP TRIGGER update_cat_updated_at ON categories;
DROP TRIGGER update_tsk_updated_at ON tasks;
DROP FUNCTION IF EXISTS update_updated_at_column();