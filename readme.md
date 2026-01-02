# Rust API Example

A simple RESTful API built with [Axum](https://github.com/tokio-rs/axum), [SQLx](https://github.com/launchbadge/sqlx), and PostgreSQL. This project demonstrates a clean architecture approach with domain, usecase, adapter, and infrastructure layers.

## Features
- User CRUD operations (create, read, update, delete)
- PostgreSQL database with SQLx
- Async runtime with Tokio
- CORS and request tracing via Tower HTTP
- Environment variable configuration
- Modular, testable architecture

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/) (for PostgreSQL)

### Setup

1. **Clone the repository:**
   ```sh
   git clone <repo-url>
   cd rust-api
   ```
2. **Start PostgreSQL with Docker Compose:**
   ```sh
   docker-compose up -d
   ```
3. **Set up environment variables:**
   Copy `.env` template and edit if needed:
   ```sh
   cp template.env .env
   ```
4. **Run database migrations:**
   ```sh
   psql $(grep DATABASE_URL .env | cut -d '=' -f2) -f migrations/script.sql
   ```
5. **Build and run the API:**
   ```sh
   cargo run
   ```

The API will be available at [http://localhost:3000](http://localhost:3000)

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string (see `template.env`)
- `RUST_LOG` - Logging level (default: `info`)

## API Endpoints

### Health Check
- `GET /health` → `200 OK` if running

### Users
- `POST /users` - Create a new user
  - Body: `{ "username": string, "email": string, "active"?: bool }`
- `GET /users` - List users
  - Query: `active` (bool), `limit` (int), `offset` (int)
- `GET /users/:id` - Get user by ID
- `PUT /users/:id` - Update user
  - Body: `{ "username"?: string, "email"?: string, "active"?: bool }`
- `DELETE /users/:id` - Delete user

#### Example User Object
```json
{
  "id": "uuid",
  "username": "alice",
  "email": "alice@example.com",
  "active": true,
  "created_at": "2026-01-02T12:00:00Z",
  "updated_at": "2026-01-02T12:00:00Z"
}
```

## Database Schema

```sql
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  active BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

## Development
- Code is organized by domain, usecase, adapters (db/http), and infrastructure layers.
- Main dependencies: axum, sqlx, tokio, serde, tower-http, tracing
- See `Cargo.toml` for full dependency list.

## License
MIT
