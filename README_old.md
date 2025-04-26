# Guardian Node

This repository contains the code for the Gridlock Guardian Node implementation.

## Prerequisites

- Docker and Docker Compose (for containerized usage)
- Rust compiler (for local builds)
- GCC, GMP, OpenSSL, and pkg-config (for native builds)

For Ubuntu/Debian, install dependencies with:

```bash
apt install -y build-essential libgmp-dev libpq-dev libssl-dev pkg-config
```

To get the Rust compiler, visit [rustup.rs](https://rustup.rs/).

## Quick Start

### Using Docker (recommended)

1. Copy the environment template:

   ```bash
   cp .env.tmpl .env
   ```

2. Build and run the guardian nodes:
   ```bash
   make run
   ```

This will start three guardian nodes and a NATS server in Docker containers.

### Building Locally

To build the guardian node locally:

```bash
make build
```

To run the guardian node locally:

```bash
make run_local
```

## Project Structure

- `backend/node` - Core node functionality library
- `backend/server-node` - Guardian node executable wrapper
- `backend/shared` - Shared definitions and utilities

## Creating a New Version

To build a new version of the guardian node:

```bash
./build_new_guardian.sh
```

This will increment the version number in Cargo.toml and build a Docker image with the new version tag.
