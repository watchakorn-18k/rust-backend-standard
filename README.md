# Rust Backend Standard Template

A production-ready Rust backend template using Axum, MongoDB, and Redis.

## Features

- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Database**: [MongoDB](https://www.mongodb.com/)
- **Cache/Store**: [Redis](https://redis.io/) (with Multiplexed Connection)
- **Config Management**: [Figment](https://github.com/SergioBenitez/Figment) (Environment & Toml)
- **Logging**: [Tracing](https://github.com/tokio-rs/tracing)
- **Documentation**: [Swagger UI/Scalar](https://github.com/scalar/scalar) (available via `/docs`)
- **Containerization**: Podman/Docker Compose support

## Project Structure

```text
.
├── src/
│   ├── handlers/      # Request handlers (Controllers)
│   ├── routes/        # Route definitions
│   ├── middlewares/   # Custom middlewares (Auth, Guards)
│   ├── models/        # Database models (BSON)
│   ├── dtos/          # Data Transfer Objects
│   ├── providers/     # External service clients (Redis, S3, Email)
│   ├── repositories/  # Database access layer
│   ├── services/      # Business logic
│   ├── config.rs      # Configuration loading
│   ├── state.rs       # Application state
│   ├── main.rs        # Entry point
│   └── error.rs       # Global error handling
├── docs/              # API documentation (Swagger/YAML)
├── tests/             # Integration tests
├── .env               # Local environment variables
└── docker-compose.yml # Infrastructure (Mongo, Redis)
```

## Getting Started

### 1. Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.75+)
- [Podman](https://podman.io/) or [Docker](https://www.docker.com/)

### 2. Setup Infrastructure
Start MongoDB and Redis:
```bash
podman-compose up -d
```

### 3. Environment Configuration
Copy the example environment file:
```bash
cp .env.example .env
```
Key configurations:
- `MONGODB_URI`: MongoDB connection string
- `REDIS_HOST`: Redis host address
- `JWT_SECRET`: Secret key for token signing

### 4. Run the Application
```bash
cargo run
```
The server will start at `http://localhost:3000`.

## API Testing

### Health Check
Verify connections to Database and Redis:
```bash
curl http://localhost:3000/health
```

### WebSocket Test
A simple WebSocket handler is available at `/ws`. You can test it using the provided `ws_test.html` or any WebSocket client.

## Development

### Hot Reload
Use `cargo-watch` for automatic reloading during development:
```bash
cargo watch -x run
```

### Linting
```bash
cargo clippy
```
