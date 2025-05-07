[← Back to main documentation](README.md)

# Customization and Development Guide

This guide provides detailed instructions for customizing and developing the Gridlock Guardian Node.

## Prerequisites

Setting up the guardian node requires the [Orchestration Node](https://github.com/GridlockNetwork/orch-node) and its associated services to be running.

## Local Development

To get started with development:

1. Copy the example configuration file:

```sh
cp example.env .env
```

2. Edit the `.env` file to customize settings:

   - Change `NATS_NETWORK=nats://nats-main:4222` to `NATS_NETWORK=nats://localhost:4222` for local development
   - Adjust other settings as needed for your environment

3. Run the project:

```sh
cargo run --bin guardian-node
```
