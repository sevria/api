# Sevria API

REST API server for Sevria, an open-source headless CMS.

## Project structure

```
api/
├── .github/            # GitHub-specific configurations
├── .vscode/            # VS Code workspace settings
├── migrations/         # SQL migrations
├── src/                # Main application source code
│   ├── config/         # Application configuration
│   ├── constant/       # Global constants
│   ├── context/        # Dependency injection and app-wide service initialization
│   ├── domain/         # Core business logic (models, repositories, services, HTTP routes)
│   ├── http/           # HTTP router that aggregates domain HTTP routes
│   └── util/           # Utility functions and helpers
└── tests/              # Integration and end-to-end tests
```

## Running tests

```bash
# Install PostgreSQL client (if not already installed)
# This allows us to seed the database using psql
sudo apt update
sudo apt install postgresql-client

# Run tests
make test
```
