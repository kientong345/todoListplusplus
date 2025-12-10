-- Add migration script here

-- ENUM types
CREATE TYPE task_status AS ENUM ('open', 'cancel', 'done');

-- USERS
CREATE TABLE IF NOT EXISTS users (
    usr_id              SERIAL PRIMARY KEY,
    usr_google_id       TEXT UNIQUE,
    usr_display_name    VARCHAR(100) NOT NULL,
    usr_email           VARCHAR(100) UNIQUE NOT NULL,
    usr_password_hash   TEXT,
    usr_avatar_url      TEXT,
    usr_description     TEXT,
    usr_created_at      TIMESTAMPTZ DEFAULT NOW(),
    usr_updated_at      TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT check_usr_email CHECK (usr_email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    CONSTRAINT check_auth_method CHECK ((usr_password_hash IS NOT NULL) OR (usr_google_id IS NOT NULL))
);

-- CATEGORIES
CREATE TABLE IF NOT EXISTS categories (
    cat_id              SERIAL PRIMARY KEY,
    cat_usr_id          INT NOT NULL REFERENCES users(usr_id) ON DELETE CASCADE,
    cat_name            TEXT NOT NULL,
    cat_image_url       TEXT,
    cat_description     TEXT,
    cat_created_at      TIMESTAMPTZ DEFAULT NOW(),
    cat_updated_at      TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT unique_cat_name_per_user UNIQUE (cat_usr_id, cat_name)
);

-- TASKS
CREATE TABLE IF NOT EXISTS tasks (
    tsk_id              SERIAL PRIMARY KEY,
    tsk_cat_id          INT NOT NULL REFERENCES categories(cat_id) ON DELETE CASCADE,
    tsk_title           TEXT NOT NULL,
    tsk_description     TEXT,
    tsk_status          task_status NOT NULL DEFAULT 'open',
    tsk_usr_comment     TEXT DEFAULT NULL,
    tsk_created_at      TIMESTAMPTZ DEFAULT NOW(),
    tsk_updated_at      TIMESTAMPTZ DEFAULT NOW(),
    tsk_expires_at      TIMESTAMPTZ DEFAULT NULL,
    tsk_reschedule_at   TIMESTAMPTZ DEFAULT NULL
);
