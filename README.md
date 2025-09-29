````markdown
# API Service Example

Пример простого REST API сервиса на **Rust** с использованием фреймворка [Axum](https://github.com/tokio-rs/axum) и базы данных **SQLite** через [SQLx](https://github.com/launchbadge/sqlx).

## Возможности
- CRUD операции для сущностей (`todos`)
- Обработка ошибок с помощью собственного `Error` типа
- Миграции базы данных
- Асинхронный сервер на Tokio

## Требования
- Rust (>=1.75)
- Cargo
- SQLite

## Установка и запуск
```bash
git clone https://github.com/kkozlovsky/api-service-example.git
cd api-service-example
cargo run
````

Сервис по умолчанию поднимается на `http://127.0.0.1:3000`.

## Миграции

Миграции находятся в папке `migrations/`.

## Структура проекта

```
src/           # код сервиса (хэндлеры, модели, ошибки)
migrations/    # SQL миграции
db.sqlite      # локальная база данных
```

## Примеры API

### Ping
```bash
curl http://127.0.0.1:3000/ping
```

### Получить список всех элементов

```bash
curl http://127.0.0.1:3000/v1/todos
```

### Получить элемент по ID

```bash
curl http://127.0.0.1:3000/v1/todos/1
```

### Создать новый элемент

```bash
curl -X POST http://127.0.0.1:3000/v1/todos \
  -H "Content-Type: application/json" \
  -d '{
    "body": "play chess"
}'
```

### Обновить элемент

```bash
curl -X PUT http://127.0.0.1:3000/v1/todos/1 \
  -H "Content-Type: application/json" \
  -d '{
    "body": "play guitar",
    "completed": true
}'
```

### Удалить элемент

```bash
curl -X DELETE http://127.0.0.1:3000/v1/todos/1
```
