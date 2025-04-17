.EXPORT_ALL_VARIABLES:
DOCKER_BUILDKIT=1
STORAGE_DIR=./storage

# Build the guardian node image and start the containers
run:
	docker build -t guardian-node:latest .
	docker compose up --remove-orphans

# Build Docker image only
build_docker:
	docker build -t guardian-node:latest .

# Build the binary
build:
	cargo build --bin guardian-node

# Run directly without Docker
run_local:
	cargo run --bin guardian-node

# Build a new guardian node version and tag it
build_new_version:
	./build_new_guardian.sh