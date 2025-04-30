# Troubleshooting Guide

## Orchestration Node Requirements

The Guardian Node requires the [Orchestration Node](https://github.com/gridlocknetwork/orch-node) to function properly. From the Orchestration Node repository, you need three main components:

1. The Orchestration Node itself
2. The networking layer
3. The database layer

## NATS Connection Issues

A common error you might encounter is:

```
Node start was unsuccessful: Failed to connect to NATS at "nats://nats-main:4222" due to error: no servers remaining to connect to
```

This error occurs when:

- NATS is not running
- The Guardian Node cannot reach the NATS server

## Network Configuration Options

There are three main ways to configure the network setup:

### 1. Docker Network Setup (Default)

- All components run in separate containers
- Components include:
  - Guardian Node
  - Orchestration Node
  - NATS
  - MongoDB
- All containers are placed in the same Docker network
- Connection string example: `nats://nats-main:4222`

### 2. Local Development Setup

- Components run directly on your local machine
- Everything connects via localhost
- Connection string example: `nats://localhost:4222`

### 3. Internet-based Setup

- Components are deployed to different servers on the internet
- Connection via public domain names
- Connection string example: `nats://nats.example.com:4222`
