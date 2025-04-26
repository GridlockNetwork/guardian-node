# syntax=docker/dockerfile:1
FROM rust:latest AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY backend/node ./backend/node
COPY backend/shared ./backend/shared
COPY backend/server-node ./backend/server-node

# Build guardian-node in debug mode
RUN cargo build --bin guardian-node
RUN cp /app/target/debug/guardian-node /app/guardian-node

FROM ubuntu:22.04
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/guardian-node /usr/local/bin/app
ENTRYPOINT ["/usr/local/bin/app"]