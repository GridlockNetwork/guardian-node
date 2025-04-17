# syntax=docker/dockerfile:1
FROM rust:latest AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Production stage
FROM ubuntu:22.04

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /app/target/release/guardian-node /app/guardian-node
COPY .env.dev /app/.env.default

# Create directories
RUN mkdir -p /var/lib/gridlock/node

# Create startup script to handle config
RUN echo '#!/bin/sh' > /app/start.sh && \
    echo 'if [ -f "/app/.env" ]; then' >> /app/start.sh && \
    echo '  echo "Using mounted .env configuration"' >> /app/start.sh && \
    echo 'else' >> /app/start.sh && \
    echo '  echo "No config mounted. Using default config."' >> /app/start.sh && \
    echo '  echo "For custom config, mount your .env file: docker run -v /path/to/.env:/app/.env ..."' >> /app/start.sh && \
    echo '  cp /app/.env.default /app/.env' >> /app/start.sh && \
    echo 'fi' >> /app/start.sh && \
    echo 'exec "$@"' >> /app/start.sh && \
    chmod +x /app/start.sh

# Expose relevant ports (adjust as needed)
EXPOSE 8080

ENTRYPOINT ["/app/start.sh"]
CMD ["/app/guardian-node"]
