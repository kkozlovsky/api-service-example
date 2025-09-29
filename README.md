# API Service Example

An example of a simple REST API service in **Rust** using the [Axum](https://github.com/tokio-rs/axum) framework and **SQLite** database via [SQLx](https://github.com/launchbadge/sqlx).

## Features
- CRUD operations for entities (`todos`)
- Error handling with a custom `Error` type
- Database migrations
- Asynchronous server powered by Tokio

## Requirements
- Rust (>=1.75)
- Cargo
- SQLite

## Installation & Run
```bash
git clone https://github.com/kkozlovsky/api-service-example.git
cd api-service-example
cargo run
```

The service runs by default at `http://127.0.0.1:3000`.

## Migrations

Migrations are located in the `migrations/` directory.

## Project Structure

```
src/           # service code (handlers, models, errors)
migrations/    # SQL migrations
db.sqlite      # local database
```

## API Examples

### Ping

```bash
curl http://127.0.0.1:3000/ping
```

### Get all todos

```bash
curl http://127.0.0.1:3000/v1/todos
```

### Get todo by ID

```bash
curl http://127.0.0.1:3000/v1/todos/1
```

### Create a new todo

```bash
curl -X POST http://127.0.0.1:3000/v1/todos \
  -H "Content-Type: application/json" \
  -d '{
    "body": "play chess"
}'
```

### Update a todo

```bash
curl -X PUT http://127.0.0.1:3000/v1/todos/1 \
  -H "Content-Type: application/json" \
  -d '{
    "body": "play guitar",
    "completed": true
}'
```

### Delete a todo

```bash
curl -X DELETE http://127.0.0.1:3000/v1/todos/1
```