# Database Schema

## Tables

### `users`
Stores user information.

| Column | Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `usr_id` | `UUID` | `PRIMARY KEY`, `DEFAULT gen_random_uuid()` | Unique identifier for the user. |
| `usr_google_id` | `TEXT` | `UNIQUE` | Google ID for OAuth users. |
| `usr_display_name` | `VARCHAR(100)` | `NOT NULL` | User's display name. |
| `usr_email` | `VARCHAR(100)` | `UNIQUE`, `NOT NULL` | User's email address. |
| `usr_password_hash` | `TEXT` | | Hashed password for local users. |
| `usr_avatar_url` | `TEXT` | | URL to the user's avatar image. |
| `usr_description` | `TEXT` | | User's bio or description. |
| `usr_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the user was created. |
| `usr_updated_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the user was last updated. |

**Constraints:**
- `check_usr_email`: Validates email format.
- `check_auth_method`: Ensures either `usr_password_hash` or `usr_google_id` is present.

---

### `categories`
Stores task categories for users.

| Column | Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `cat_id` | `UUID` | `PRIMARY KEY`, `DEFAULT gen_random_uuid()` | Unique identifier for the category. |
| `cat_usr_id` | `UUID` | `NOT NULL`, `REFERENCES users(usr_id) ON DELETE CASCADE` | ID of the user who owns the category. |
| `cat_name` | `TEXT` | `NOT NULL` | Name of the category. |
| `cat_image_url` | `TEXT` | | URL to the category's image. |
| `cat_description` | `TEXT` | | Description of the category. |
| `cat_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the category was created. |
| `cat_updated_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the category was last updated. |

**Constraints:**
- `unique_cat_name_per_user`: Ensures `cat_name` is unique per user.

---

### `task_chains`
Represents the primary definition of a task, which can have multiple occurrences (links) over time.

| Column | Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `chain_id` | `UUID` | `PRIMARY KEY`, `DEFAULT gen_random_uuid()` | Unique identifier for the task chain. |
| `chain_cat_id` | `UUID` | `NOT NULL`, `REFERENCES categories(cat_id) ON DELETE CASCADE` | ID of the category the task belongs to. |
| `chain_title` | `TEXT` | `NOT NULL` | Title of the task. |
| `chain_description` | `TEXT` | | Detailed description of the task. |
| `chain_usr_comment` | `TEXT` | `DEFAULT NULL` | User's comment on the task chain. |
| `chain_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the chain was created. |
| `chain_updated_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the chain was last updated. |
| `chain_cycle_time` | `INTERVAL` | `DEFAULT NULL` | Cycle time for recurring tasks. |
| `chain_terminated_at` | `TIMESTAMPTZ` | `DEFAULT NULL` | Timestamp if the chain has been terminated. |

**Constraints:**
- `unique_chain_per_category`: Ensures `chain_title` is unique within a category.

---

### `task_chain_links`
Represents specific occurrences or instances of a task defined in `task_chains`.

| Column | Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `link_id` | `SERIAL` | `PRIMARY KEY` | Unique identifier for the link. |
| `link_chain_id` | `UUID` | `NOT NULL`, `REFERENCES task_chains(chain_id) ON DELETE CASCADE` | Reference to the parent task chain. |
| `link_status` | `task_status` | `NOT NULL`, `DEFAULT 'open'` | Status of this specific occurrence. |
| `link_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | When this link was created. |
| `link_expires_at` | `TIMESTAMPTZ` | `DEFAULT NULL` | Expiration time for this instance. |
| `link_notify_time` | `TIMESTAMPTZ` | `DEFAULT NULL` | Notification time for this instance. |
| `link_prev_id` | `INT` | `REFERENCES task_chain_links(link_id) ON DELETE SET NULL` | Reference to the previous link in the chain. |
| `link_is_latest` | `BOOLEAN` | `NOT NULL`, `DEFAULT TRUE` | Flag indicating if this is the active link. |

---

## Enums

### `task_status`
- `open`
- `cancel`
- `done`

---

## Views

### `tasks`
A convenience view that combines `task_chains` and the latest `task_chain_links` to represent the current state of all active tasks.

| Column | Type | Origin |
| :--- | :--- | :--- |
| `tsk_id` | `UUID` | `chain_id` |
| `tsk_cat_id` | `UUID` | `chain_cat_id` |
| `tsk_title` | `TEXT` | `chain_title` |
| `tsk_description` | `TEXT` | `chain_description` |
| `tsk_status` | `task_status` | `link_status` |
| `tsk_usr_comment` | `TEXT` | `chain_usr_comment` |
| `tsk_created_at` | `TIMESTAMPTZ` | `chain_created_at` |
| `tsk_updated_at` | `TIMESTAMPTZ` | `chain_updated_at` |
| `tsk_expires_at` | `TIMESTAMPTZ` | `link_expires_at` |
| `tsk_cycle_time` | `INTERVAL` | `chain_cycle_time` |
| `tsk_notify_time` | `TIMESTAMPTZ` | `link_notify_time` |

---

### `categories_with_tasks_count`
Provides category details along with task statistics (aggregated from the `tasks` view).

| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | `UUID` | Category ID (`cat_id`). |
| `user_id` | `UUID` | User ID (`cat_usr_id`). |
| `name` | `TEXT` | Category name (`cat_name`). |
| `image_url` | `TEXT` | Category image URL (`cat_image_url`). |
| `description` | `TEXT` | Category description (`cat_description`). |
| `created_at` | `TIMESTAMPTZ` | Category creation timestamp (`cat_created_at`). |
| `updated_at` | `TIMESTAMPTZ` | Category update timestamp (`cat_updated_at`). |
| `opened_task_count` | `BIGINT` | Count of open tasks. |
| `canceled_task_count` | `BIGINT` | Count of canceled tasks. |
| `done_task_count` | `BIGINT` | Count of done tasks. |
| `task_count` | `BIGINT` | Total count of tasks. |
| `progress` | `DOUBLE PRECISION` | Progress percentage (done / (open + done)). |

---

## Triggers

### `update_updated_at_column`
Automatically updates the `*_updated_at` column to `NOW()` before an update operation on `users`, `categories`, and `task_chains` tables.
