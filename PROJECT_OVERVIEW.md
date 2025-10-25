# 📘 Knowledge-Notebook: Project Overview and Roadmap

## 🧩 Project Summary

**Knowledge-Notebook** is a Rust-based backend service that provides a secure and modular foundation for a personal or team note-taking system.
It currently includes:

* **REST API** built with **Axum**
* **PostgreSQL** database managed via **SQLx**
* **JWT authentication** with user registration and login
* **Nix-based reproducible build environment** for consistent development
* **Docker-based** local deployment for backend + database
* Environment variable management via `.env`
* Planned **Deno Fresh** frontend integration

---

## 🧱 Core Architecture

| Component              | Description                                                     |
| ---------------------- | --------------------------------------------------------------- |
| **Language**           | Rust (Axum + SQLx + Tokio async runtime)                        |
| **Database**           | PostgreSQL (UUID primary keys, timestamped records)             |
| **Auth**               | JWT tokens with validation extractor                            |
| **Password security**  | Argon2 hashing                                                  |
| **Environment**        | `.env` for sensitive settings like `DATABASE_URL`, `JWT_SECRET` |
| **Deployment**         | Docker Compose for backend + database                           |
| **Build**              | Nix flake environment for consistent dev tools and deps         |
| **Frontend (planned)** | Deno + Fresh for SSR-based reactive UI                          |
| **Logging & Errors**   | Anyhow + tracing for error context and debug logging            |

---

## 🔐 Current Features

### ✅ Authentication

* JWT-based user sessions with 24-hour expiration.
* `AuthUser` extractor to protect routes.
* `JWT_SECRET` validation on startup.

### 👥 User Management

* Register (`POST /auth/register`)
* Login (`POST /auth/login`) → returns JWT token
* List users (`GET /users`) → protected route, requires valid token

**Users Table:**

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## 📒 Planned Next Phase: Notes Module

**Goal:** Introduce user-owned notes.

### Proposed Schema:

```sql
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Planned Endpoints:

| Method   | Path         | Description                       |
| -------- | ------------ | --------------------------------- |
| `GET`    | `/notes`     | List notes for authenticated user |
| `POST`   | `/notes`     | Create a new note                 |
| `PUT`    | `/notes/:id` | Update a note                     |
| `DELETE` | `/notes/:id` | Delete a note                     |

All protected with the `AuthUser` extractor.

---

## ⚙️ Infrastructure & Tooling

### **Nix Environment**

The project uses **Nix** for reproducible builds and dependency management.

**Features:**

* Reproducible Rust toolchain
* Built-in PostgreSQL, SQLx CLI, and Deno runtime
* Consistent development environment for all contributors

**Example usage:**

```bash
nix develop
cargo run
```

### **Migrations**

Located at top-level `/migrations` folder.
Use:

```bash
sqlx migrate add create_notes_table
sqlx migrate run
```

### **Docker**

Local development uses:

* `db` container for PostgreSQL
* Optional `backend` container for Rust API server (if added)

`.env` should include:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/notebook_dev
JWT_SECRET=your_long_random_secret
```

### **Startup Validation**

Main function checks for the presence of `JWT_SECRET` to prevent insecure defaults.

---

## 🌊 Planned Frontend: Deno + Fresh

**Technology:** [Fresh](https://fresh.deno.dev/) — a Deno-based SSR web framework.

**Benefits:**

* TypeScript-first design
* Zero-build server-side rendering
* Native integration with REST APIs

**Planned Structure:**

```
frontend/
├── routes/
│   ├── index.tsx
│   ├── notes.tsx
│   └── login.tsx
├── components/
├── islands/
└── deno.json
```

**Integration:** Communicates with Rust backend at `http://localhost:3000` via REST endpoints.

---

## 🦯 Roadmap

| Phase | Focus                                | Status    |
| ----- | ------------------------------------ | --------- |
| 1     | Setup database, backend skeleton     | ✅ Done    |
| 2     | Implement user registration & login  | ✅ Done    |
| 3     | Add JWT auth extractor               | ✅ Done    |
| 4     | Secure routes with AuthUser          | ✅ Done    |
| 5     | Add `notes` module (CRUD, ownership) | 🚧 Next   |
| 6     | Testing & error refactor             | ⏳ Planned |
| 7     | Setup Deno Fresh frontend            | ⏳ Planned |
| 8     | Integrate frontend + backend         | ⏳ Planned |

---

## 📚 Repository Reference

GitHub: [spog/knowledge-notebook](https://github.com/spog/knowledge-notebook)
Structure (relevant part):

```
knowledge-notebook/
│
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── auth.rs
│   │   ├── users.rs
│   │   ├── models.rs
│   │   ├── utils.rs
│   │   └── ...
│   ├── Cargo.toml
│
├── migrations/
│   ├── 20231001120000_create_users.sql
│   └── 20231002120000_create_notes.sql  # upcoming
│
├── frontend/  # Deno Fresh app (planned)
│
├── docker-compose.yml
├── flake.nix  # Nix build configuration
└── .env
```

---

## 🚀 Next Steps

1. ✅ Confirm current backend build and auth works.
2. 📒 Implement the `notes` module with CRUD endpoints.
3. ⚙️ Add tests for auth & users.
4. 🧪 Define Nix build/test scripts for CI.
5. 🌊 Initialize Fresh frontend project.
6. 🗾 Improve error responses (standardized JSON error format).

