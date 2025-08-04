# Sevria API

REST API server for Sevria, an open-source headless CMS.

## Features

- Written in Rust for performance and safety
- Clean architecture (feature-based structure)
- JWT authentication
- OpenAPI documentation

## Project structure

```
api/
├── .github/          # GitHub-specific configurations
├── .vscode/          # VS Code workspace settings
├── migrations/       # SQL migrations
├── src/              # Main application source code
│   ├── config/       # Application configuration
│   ├── constant/     # Global constants
│   ├── context/      # Dependency injection and app-wide service initialization
│   ├── domain/       # Core business logic (models, repositories, services, HTTP routes)
│   ├── http/         # HTTP router that aggregates domain HTTP routes
│   └── util/         # Utility functions and helpers
└── tests/            # Integration and end-to-end tests
```

## Getting started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/)
- [sqlx-cli](https://crates.io/crates/sqlx-cli) (for migrations)
- [psql](https://www.tigerdata.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows) (for seeding test data)

### Setup

Clone the repository:

```shell
git clone https://github.com/sevria/api.git sevria-api
cd sevria-api
```

Copy `.env.example` and edit values:

```shell
cp .env.example .env
```

Run migrations:

```shell
sqlx migrate run
```

Build and run:

```shell
cargo run
```

### Environment variables

| Variable                 | Description                    | Default value       |
| ------------------------ | ------------------------------ | ------------------- |
| `CORS_ALLOW_ORIGIN`      | Allowed CORS origins           | `*`                 |
| `DATABASE_URL`           | PostgreSQL connection string   | N/A                 |
| `DEFAULT_ADMIN_EMAIL`    | Default admin user email       | `admin@example.com` |
| `DEFAULT_ADMIN_PASSWORD` | Default admin password         | `Sevria123`         |
| `HTTP_ADDRESS`           | Server bind address            | `0.0.0.0:4000`      |
| `JWT_EXPIRES_IN_MINUTES` | JWT expiration time in minutes | `15`                |
| `JWT_SECRET`             | Secret key used to sign JWTs   | N/A                 |

### API docs

The OpenAPI documentation is available at http://localhost:4000/docs when the application is running.

### Running tests

Integration tests are executed locally and will recreate the test database. This includes dropping the existing database (if it exists) and creating a new one to ensure a clean state for every test run.

Make sure your database user has sufficient privileges (e.g. the ability to drop and create databases). Using a superuser like postgres is recommended during testing.

To run integration tests:

```bash
make test
```

## License

This project is licensed under the MIT License. See `LICENSE` for more information.
