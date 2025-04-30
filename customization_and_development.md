[← Back to main documentation](README.md)

# Customization and Development Guide

This guide provides detailed instructions for customizing and developing the Gridlock Guardian Node.

## Prerequisites

Setting up the guardian node requires the [Orchestration Node](https://github.com/GridlockNetwork/orch-node) and its associated services to be running.

## Configuration

The application supports two configuration options:

1. Default config (baked into the image from example.env)
2. User config (overrides default)

We recommend storing your config file at the absolute path: `/Users/USERNAME/.gridlock-guardian-node/.env` (replace `USERNAME` with your actual username).

To run with a custom configuration:

```sh
docker run --rm --name guardian-node --network gridlock-net \
  -v /Users/USERNAME/.gridlock-guardian-node/.env:/app/.env \
  gridlocknetwork/guardian-node:latest
```

## Local Development Setup

To run the project locally, copy and run these commands:

```sh
npm install
npm run compile
npm run dev
```

## Customizing Docker Compose

The default docker-compose setup uses standard configurations. To customize:

1. Create a `docker-compose.override.yml` file
2. Add your custom configurations
3. Run `docker-compose up`

Example override file:

```yaml
version: "3.8"
services:
  guardian-node-1:
    environment:
      - CUSTOM_ENV_VAR=value
    volumes:
      - ./custom-config:/app/config
  guardian-node-2:
    environment:
      - CUSTOM_ENV_VAR=value
    volumes:
      - ./custom-config:/app/config
  guardian-node-3:
    environment:
      - CUSTOM_ENV_VAR=value
    volumes:
      - ./custom-config:/app/config
```

## How It Works

The guardian node communicates with the [orch-node](https://github.com/GridlockNetwork/orch-node) to coordinate signing and recovery. It holds one key share and uses threshold signature cryptography, meaning several guardians must work together to authorize actions. This ensures that no single node can compromise a user's wallet—even if it's lost or attacked.

The node uses internal storage inside the container and does not require additional persistent volumes.

For usage with other tools, check out:

- [gridlock-sdk](https://github.com/GridlockNetwork/gridlock-sdk)
- [gridlock-cli](https://github.com/GridlockNetwork/gridlock-cli)
