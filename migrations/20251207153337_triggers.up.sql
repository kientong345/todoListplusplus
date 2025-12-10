-- Add up migration script here
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_TABLE_NAME = 'users' THEN
        NEW.usr_updated_at = NOW();
    ELSIF TG_TABLE_NAME = 'categories' THEN
        NEW.cat_updated_at = NOW();
    ELSIF TG_TABLE_NAME = 'tasks' THEN
        NEW.tsk_updated_at = NOW();
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Sau đó tạo triggers
CREATE TRIGGER update_usr_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_cat_updated_at
    BEFORE UPDATE ON categories
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tsk_updated_at
    BEFORE UPDATE ON tasks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();