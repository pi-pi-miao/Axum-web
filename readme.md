```markdown
# axum-web

A CRUD example using Axum web framework with MySQL database integration.

## Features

- RESTful CRUD operations
- MySQL database integration using SQLx
- Custom middleware implementation
- Custom error handling
- Custom response structures
- Tracing for logging (console and file output)
- Structured JSON responses
- Query parameter handling
- Path parameter extraction
- JSON body parsing

## Prerequisites

- Rust (latest stable version)
- MySQL server
- Cargo and Rust toolchain

## Setup

1. Clone the repository:
   git clone https://github.com/yourusername/axum-web.git
   cd axum-web


2. Set up your MySQL database and update the connection string in `config.rs` or use environment variables.

3. Install dependencies:

cargo build

4. Run the application:

cargo run

## API Endpoints

- `GET /`: Health check endpoint
- `GET /tickets`: List all tickets
- `GET /tickets/:id`: Get a specific ticket
- `POST /tickets`: Create a new ticket
- `PUT /tickets/:id`: Update an existing ticket
- `DELETE /tickets/:id`: Delete a ticket
- `POST /tickets/batch`: Batch insert tickets

## Configuration

Environment variables:

- `DATABASE_URL`: MySQL connection string
- `SERVER_ADDR`: Server address and port (default: "127.0.0.1:8080")
- `LOG_LEVEL`: Logging level (default: "debug")

## Logging

Logs are output to both console and file (`./logs/axum-log-yyyy-mm-dd`). Log rotation is implemented to manage log file
size.

## Error Handling

Custom error types are defined in `error.rs`, providing structured error responses.

## Response Format

All API responses follow a consistent JSON format:

```json

```

## Testing

Run the test suite with:

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

