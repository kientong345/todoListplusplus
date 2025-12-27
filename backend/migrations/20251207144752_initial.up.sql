-- Add migration script here

-- ENUM types
CREATE TYPE task_status AS ENUM ('open', 'cancel', 'done');

-- USERS
CREATE TABLE IF NOT EXISTS users (
    usr_id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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
    cat_id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cat_usr_id          UUID NOT NULL REFERENCES users(usr_id) ON DELETE CASCADE,
    cat_name            TEXT NOT NULL,
    cat_image_url       TEXT,
    cat_description     TEXT,
    cat_created_at      TIMESTAMPTZ DEFAULT NOW(),
    cat_updated_at      TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT unique_cat_name_per_user UNIQUE (cat_usr_id, cat_name)
);

-- TASK_CHAINS
CREATE TABLE IF NOT EXISTS task_chains (
    chain_id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chain_cat_id            UUID NOT NULL REFERENCES categories(cat_id) ON DELETE CASCADE,
    chain_title             TEXT NOT NULL,
    chain_description       TEXT,
    chain_usr_comment       TEXT DEFAULT NULL,
    chain_created_at        TIMESTAMPTZ DEFAULT NOW(),
    chain_updated_at        TIMESTAMPTZ DEFAULT NOW(),
    chain_cycle_time        INTERVAL DEFAULT NULL,
    chain_terminated_at     TIMESTAMPTZ DEFAULT NULL,

    CONSTRAINT unique_chain_per_category UNIQUE (chain_cat_id, chain_title)
);

-- TASK_CHAIN_LINKS
CREATE TABLE IF NOT EXISTS task_chain_links (
    link_id             SERIAL PRIMARY KEY,
    link_status         task_status NOT NULL DEFAULT 'open',
    link_created_at     TIMESTAMPTZ DEFAULT NOW(),
    link_expires_at     TIMESTAMPTZ DEFAULT NULL,
    link_notify_time    TIMESTAMPTZ DEFAULT NULL,
    link_chain_id       UUID NOT NULL REFERENCES task_chains(chain_id) ON DELETE CASCADE,
    link_prev_id        INT DEFAULT NULL REFERENCES task_chain_links(link_id) ON DELETE SET NULL,
    link_is_latest      BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE OR REPLACE VIEW tasks AS
SELECT
    chains.chain_id AS tsk_id,
    chains.chain_cat_id AS tsk_cat_id,
    chains.chain_title AS tsk_title,
    chains.chain_description AS tsk_description,
    links.link_status AS tsk_status,
    chains.chain_usr_comment AS tsk_usr_comment,
    chains.chain_created_at AS tsk_created_at,
    chains.chain_updated_at AS tsk_updated_at,
    links.link_expires_at AS tsk_expires_at,
    chains.chain_cycle_time AS tsk_cycle_time,
    links.link_notify_time AS tsk_notify_time
FROM task_chains AS chains
LEFT JOIN task_chain_links AS links ON chains.chain_id = links.link_chain_id
WHERE links.link_is_latest = TRUE AND chains.chain_terminated_at IS NULL;