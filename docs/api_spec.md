# API Specification

## Authentication

### Register
`POST /api/v1/auth/register`

**Request Body:**
```json
{
  "displayName": "John Doe",
  "email": "john@example.com",
  "password": "securepassword"
}
```

**Response:**
- `201 Created`: Registration successful.
- `400 Bad Request`: Invalid input (e.g., invalid email).

### Login
`POST /api/v1/auth/login`

**Request Body:**
```json
{
  "email": "john@example.com",
  "password": "securepassword"
}
```

**Response:**
- `200 OK`: Login successful.
  ```json
  {
    "access_token": "jwt_token_here"
  }
  ```
  *Note: Refresh token is set in a `HttpOnly` cookie.*

### Google Login
`POST /api/v1/auth/google-login`

**Query Parameters:**
- `code`: Authorization code from Google.
- `state`: State parameter for CSRF protection.

**Response:**
- `200 OK`: Login successful.
  ```json
  {
    "access_token": "jwt_token_here"
  }
  ```
  *Note: Refresh token is set in a `HttpOnly` cookie.*

---

## Users

### Get Current User
`GET /api/v1/users/me`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Response:**
- `200 OK`
  ```json
  {
    "id": 1,
    "displayName": "John Doe",
    "email": "john@example.com",
    "avatarUrl": "https://example.com/avatar.jpg",
    "description": "I love coding",
    "createdAt": "2023-01-01T00:00:00Z",
    "updatedAt": "2023-01-01T00:00:00Z"
  }
  ```

### Update Current User
`PATCH /api/v1/users/me`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Request Body:**
```json
{
  "displayName": "John Doe Updated",
  "avatarUrl": "https://example.com/new_avatar.jpg",
  "description": "Updated description"
}
```
*Note: All fields are optional.*

**Response:**
- `200 OK`: Update successful.

---

## Categories

### Get Categories
`GET /api/v1/categories`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Query Parameters:**
- `namePattern` (optional): Filter by name.
- `page` (required): Page number (0-indexed).
- `pageSize` (required): Number of items per page.
- `sortBy` (required): Sort field ('new-update', 'task-count', 'progress').

**Response:**
- `200 OK`
  ```json
  {
    "items": [
      {
        "id": 1,
        "userId": 1,
        "name": "Work",
        "imageUrl": "https://example.com/work.jpg",
        "description": "Work related tasks",
        "taskCount": 5
      }
    ],
    "totalItems": 1,
    "totalPages": 1,
    "page": 0,
    "pageSize": 10
  }
  ```

### Create Category
`POST /api/v1/categories`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Request Body:**
```json
{
  "name": "Personal",
  "imageUrl": "https://example.com/personal.jpg",
  "description": "Personal tasks"
}
```

**Response:**
- `201 Created`: Category created successfully.

### Get Category Details
`GET /api/v1/categories/:id`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Response:**
- `200 OK`
  ```json
  {
    "id": 1,
    "userId": 1,
    "name": "Work",
    "imageUrl": "https://example.com/work.jpg",
    "description": "Work related tasks",
    "createdAt": "2023-01-01T00:00:00Z",
    "updatedAt": "2023-01-01T00:00:00Z",
    "taskCount": 5,
    "openedTaskCount": 2,
    "canceledTaskCount": 1,
    "doneTaskCount": 2,
    "progress": 0.5
  }
  ```

### Update Category
`PATCH /api/v1/categories/:id`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Request Body:**
```json
{
  "name": "Work Updated",
  "imageUrl": null,
  "description": "Updated description"
}
```
*Note: All fields are optional.*

**Response:**
- `200 OK`: Update successful.

### Delete Category
`DELETE /api/v1/categories/:id`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Response:**
- `200 OK`: Deletion successful.

---

## Tasks

### Get Tasks
`GET /api/v1/categories/:category_id/tasks`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Query Parameters:**
- `titlePattern` (optional): Filter by title.
- `status` (optional): Filter by status (e.g., `open`, `done`). Can be multiple.
- `page` (required): Page number.
- `pageSize` (required): Page size.
- `sortBy` (required): Sort field: ('latest' | 'new-update' | 'deadline').

**Response:**
- `200 OK`
  ```json
  {
    "items": [
      {
        "id": 1,
        "title": "Finish report",
        "status": "open",
        "expiresAt": "2023-01-02T00:00:00Z",
        "cycleTime": "1 day"
      }
    ],
    "totalItems": 1,
    "totalPages": 1,
    "page": 0,
    "pageSize": 10
  }
  ```

### Create Task
`POST /api/v1/categories/:category_id/tasks`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Request Body:**
```json
{
  "title": "New Task",
  "description": "Task description",
  "expiresAt": "2023-01-02T00:00:00Z",
  "cycleTime": "1 day",
  "preNotifyTime": "1 hour"
}
```

**Response:**
- `201 Created`: Task created successfully.

### Get Task Details
`GET /api/v1/categories/:category_id/tasks/:task_id`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Response:**
- `200 OK`
  ```json
  {
    "id": 1,
    "categoryId": 1,
    "categoryName": "Work",
    "title": "Finish report",
    "description": "Task description",
    "status": "open",
    "userComment": "Important",
    "createdAt": "2023-01-01T00:00:00Z",
    "updatedAt": "2023-01-01T00:00:00Z",
    "expiresAt": "2023-01-02T00:00:00Z",
    "cycleTime": "1 day",
    "preNotifyTime": "1 hour"
  }
  ```

### Update Task
`PATCH /api/v1/categories/:category_id/tasks/:task_id`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Request Body:**
```json
{
  "title": "Updated Title",
  "description": "Updated description",
  "status": "done",
  "userComment": "Done"
}
```
*Note: All fields are optional.*

**Response:**
- `200 OK`: Update successful.

### Delete Task
`DELETE /api/v1/categories/:category_id/tasks/:task_id`

**Headers:**
- `Authorization`: `Bearer <access_token>`

**Response:**
- `200 OK`: Deletion successful.
