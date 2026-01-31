# Rust Axum MongoDB Template (2026)

A high-performance, maintainable backend template using Rust, Axum, and MongoDB.

## Features

- **Axum**: Modern, ergonomic, and fast web framework.
- **MongoDB**: Async driver with generic repository pattern.
- **Architecture**: Clean separation of concerns (Handler -> Service -> Repository).
- **Validation**: Input validation using `validator` crate.
- **Configuration**: Type-safe config loading from Environment/File.
- **Error Handling**: Centralized `AppError` enum compatible with `IntoResponse`.

## Getting Started

1.  **Start Database**:
    ```bash
    docker-compose up -d
    ```

2.  **Environment**:
    ```bash
    cp .env.example .env
    ```

3.  **Run**:
    ```bash
    cargo run
    ```

## Project Structure

- `src/handlers`: HTTP Controllers (Axum).
- `src/services`: Business Logic.
- `src/repositories`: Data Access Layer.
- `src/models`: Database Structs.
- `src/dtos`: Data Transfer Objects (Input/Output structs).
- `src/config.rs`: Configuration loader.
- `src/error.rs`: Global error handling.

## API Endpoints

- `POST /api/v1/users` - Create user
- `GET /api/v1/users` - List users (pagination supported)
- `GET /api/v1/users/:id` - Get user by ID
- `PUT /api/v1/users/:id` - Update user
