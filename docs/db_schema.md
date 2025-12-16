# Database Schema

## Tables

### `users`
Stores user information.

| Column | Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `usr_id` | `SERIAL` | `PRIMARY KEY` | Unique identifier for the user. |
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

### `categories`
Stores task categories for users.

| Column | Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `cat_id` | `SERIAL` | `PRIMARY KEY` | Unique identifier for the category. |
| `cat_usr_id` | `INT` | `NOT NULL`, `REFERENCES users(usr_id) ON DELETE CASCADE` | ID of the user who owns the category. |
| `cat_name` | `TEXT` | `NOT NULL` | Name of the category. |
| `cat_image_url` | `TEXT` | | URL to the category's image. |
| `cat_description` | `TEXT` | | Description of the category. |
| `cat_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the category was created. |
| `cat_updated_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the category was last updated. |

**Constraints:**
- `unique_cat_name_per_user`: Ensures `cat_name` is unique per user.

### `tasks`
Stores tasks within categories.

| Column | Type | Constraints | Description |
| :--- | :--- | :--- | :--- |
| `tsk_id` | `SERIAL` | `PRIMARY KEY` | Unique identifier for the task. |
| `tsk_cat_id` | `INT` | `NOT NULL`, `REFERENCES categories(cat_id) ON DELETE CASCADE` | ID of the category the task belongs to. |
| `tsk_title` | `TEXT` | `NOT NULL` | Title of the task. |
| `tsk_description` | `TEXT` | | Detailed description of the task. |
| `tsk_status` | `task_status` | `NOT NULL`, `DEFAULT 'open'` | Status of the task (`open`, `cancel`, `done`). |
| `tsk_usr_comment` | `TEXT` | `DEFAULT NULL` | User's comment on the task. |
| `tsk_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the task was created. |
| `tsk_updated_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Timestamp when the task was last updated. |
| `tsk_expires_at` | `TIMESTAMPTZ` | `DEFAULT NULL` | Expiration time of the task. |
| `tsk_cycle_time` | `INTERVAL` | `DEFAULT NULL` | Cycle time for recurring tasks. |
| `tsk_pre_notify_time` | `INTERVAL` | `DEFAULT NULL` | Time before expiration to notify the user. |
| `tsk_next_version_id` | `INT` | `DEFAULT NULL`, `REFERENCES tasks(tsk_id) ON DELETE SET NULL` | ID of the next version of the task. |

## Enums

### `task_status`
- `open`
- `cancel`
- `done`

## Views

### `categories_with_tasks_count`
Provides category details along with task statistics.

| Column | Type | Description |
| :--- | :--- | :--- |
| [id](file:///home/kt345/Documents/my_workspace/todo-list/src/model/user_auth/auth.rs#14-24) | `INT` | Category ID (`cat_id`). |
| `user_id` | `INT` | User ID (`cat_usr_id`). |
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

## Triggers

### `update_updated_at_column`
Automatically updates the `*_updated_at` column to `NOW()` before an update operation on `users`, `categories`, and `tasks` tables.
