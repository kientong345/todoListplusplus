# Todo List++

A modern, full-stack Todo List application built with performance and user experience in mind.

## 🚀 Tech Stack

### Backend
- **Language**: Rust
- **Framework**: Axum
- **Database**: PostgreSQL (with SQLx)
- **Authentication**: JWT & Google OAuth

### Frontend
- **Framework**: React (Vite)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **UI Library**: shadcn/ui
- **State Management**: React Context

## 📂 Project Structure

```
todo-list/
├── backend/      # Rust API Server
├── frontend/     # React SPA
├── docs/         # Documentation
└── ...
```

## 🛠️ Setup & Installation

### Prerequisites
- Rust (latest stable)
- Node.js (v18+)
- PostgreSQL
- Docker (optional, for DB)

### Backend Setup

1.  Navigate to the backend directory:
    ```bash
    cd backend
    ```
2.  Set up environment variables (copy `.env.example` to `.env` and update values).
3.  Run database migrations:
    ```bash
    sqlx migrate run
    ```
4.  Start the server:
    ```bash
    cargo run
    ```

### Frontend Setup

1.  Navigate to the frontend directory:
    ```bash
    cd frontend
    ```
2.  Install dependencies:
    ```bash
    npm install
    ```
3.  Start the development server:
    ```bash
    npm run dev
    ```

## ✨ Features

- **Authentication**: Secure login/register with email or Google.
- **Categories**: Organize tasks into custom categories.
- **Tasks**: Create, update, delete, and track tasks.
- **Progress Tracking**: Visual progress bars for categories.
- **Responsive Design**: Works seamlessly on desktop and mobile.

## 📖 Documentation

- [Database Schema](docs/db_schema.md) (Generated)
- [API Specification](docs/api_spec.md) (Generated)
