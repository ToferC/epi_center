# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based GraphQL API called "People Data Analytics" (package name: `people_data_api`) that models employee skills, capabilities, certifications, and organizational structures over time. It's built with:

- **Backend**: Actix-web with async-graphql for GraphQL API
- **Database**: PostgreSQL with Diesel ORM
- **Authentication**: JWT-based with Argon2 password hashing
- **Frontend**: React app in `/orgchart` directory for organizational chart visualization

## Development Commands

### Setup
```bash
# Install diesel CLI if not present
cargo install diesel_cli --no-default-features --features postgres

# Setup database
diesel migration run

# Run the application
cargo run
```

### Development with Docker
```bash
# Start PostgreSQL database
docker compose up -d db

# Wait for DB to be ready, then run migrations
sleep 10 && diesel migration run

# Build and run the application
docker compose up

# Build optimized image (uses Dockerfile.slim)
time docker compose build people-data-api

# View logs
docker compose logs -f
```

### MacOS Specific Setup
```bash
# Install PostgreSQL client libraries
brew install libpq
brew link --force libpq

# Clean and rebuild
cargo clean
```

## Architecture

### Core Structure
- **Models** (`src/models/`): Database entities including Person, Organization, Team, Role, Capability, Publication, etc.
- **GraphQL** (`src/graphql/`): Query and mutation resolvers organized by domain
- **Database** (`src/database/`): Database connection, admin operations, and dummy data generation
- **Handlers** (`src/handlers/`): HTTP endpoints and routing
- **Schema** (`src/schema.rs`): Diesel-generated database schema

### Key Components
- **Authentication**: JWT-based auth with role-based access control
- **Data Models**: Comprehensive modeling of organizational structures, skills, and work relationships
- **GraphQL API**: Full CRUD operations with subscription support
- **Database Migrations**: Located in `/migrations` directory
- **Dummy Data**: Seed data generation for development (`src/database/dummy_*.rs`)

### Frontend
- React application in `/orgchart` directory
- Organizational chart visualization
- Standard React TypeScript setup with npm

## Environment Setup

Create `.env` file with:
```
DATABASE_URL=postgres://christopherallison:12345@localhost/people_data_api?sslmode=disable
SECRET_KEY=32CHARSECRETKEY
PASSWORD_SECRET_KEY=32CHARSECRETKEY
JWT_SECRET_KEY=32CHARSECRETKEY
ADMIN_EMAIL=some_admin@email.com
ADMIN_PASSWORD=ADMINPASSWORD
ADMIN_NAME="Admin Name"
```

## Testing and Deployment

- **Docker Images**: Multiple Dockerfile variants (simple, slim, alpine) for different deployment needs
- **Kubernetes**: Deployment configs in `/kubernetes` directory
- **GKE**: Google Kubernetes Engine configs in `/gke-k8s` directory
- **Migrations**: Run via `diesel migration run` before application startup

## Database Schema

The application uses PostgreSQL with Diesel ORM. Key tables include:
- `users`, `persons`, `organizations`, `teams`
- `roles`, `capabilities`, `skills`, `publications`
- `affiliations`, `reporting_relationships`, `team_ownership`

Schema is auto-generated in `src/schema.rs` via Diesel CLI.