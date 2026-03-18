#!/bin/bash

# Configuration
IMAGE_NAME="metatron-backend"
CONTAINER_NAME="metatron-backend"
HTTP_PORT=8080
GRPC_PORT=50060
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONTEXT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Metatron Backend Manager ===${NC}"
echo "Select an action:"
echo "1) Build           — cargo build --release"
echo "2) Run Local       — run compiled binary"
echo "3) Build Image     — Docker build"
echo "4) Run Container   — Docker run (HTTP ${HTTP_PORT}, gRPC ${GRPC_PORT})"
echo "5) Build & Run     — Docker build then run"
echo "6) Stop Container  — stop and remove container"
echo "7) Clean           — cargo clean"
echo "8) Exit"
echo ""

read -p "Enter choice [1-8]: " choice

stop_container() {
    echo -e "${BLUE}Checking for existing container '${CONTAINER_NAME}'...${NC}"
    EXISTING=$(docker ps -q --filter "name=${CONTAINER_NAME}")
    if [ ! -z "$EXISTING" ]; then
        echo -e "${YELLOW}Stopping container ${EXISTING}...${NC}"
        docker rm -f "$CONTAINER_NAME"
        echo -e "${GREEN}Container stopped.${NC}"
    else
        echo -e "${BLUE}No running container found.${NC}"
    fi
}

build_binary() {
    echo -e "${GREEN}Building metatron-backend (release)...${NC}"
    cd "$SCRIPT_DIR"
    cargo build --release --bin metatron-api
    if [ $? -ne 0 ]; then
        echo -e "${RED}Build failed.${NC}"
        exit 1
    fi
    echo -e "${GREEN}Binary built: target/release/metatron-api${NC}"
}

run_local() {
    cd "$SCRIPT_DIR"
    if [ ! -f target/release/metatron-api ]; then
        echo -e "${RED}Binary not found. Build first (option 1).${NC}"
        exit 1
    fi
    echo -e "${GREEN}Starting metatron-api...${NC}"
    echo -e "${BLUE}HTTP:  http://localhost:${HTTP_PORT}${NC}"
    echo -e "${BLUE}gRPC:  localhost:${GRPC_PORT}${NC}"
    exec ./target/release/metatron-api
}

build_image() {
    echo -e "${GREEN}Building Docker image '${IMAGE_NAME}'...${NC}"
    docker build -t "$IMAGE_NAME" -f "$SCRIPT_DIR/Dockerfile" "$CONTEXT_DIR"
    if [ $? -ne 0 ]; then
        echo -e "${RED}Docker build failed.${NC}"
        exit 1
    fi
    echo -e "${GREEN}Image '${IMAGE_NAME}' built successfully.${NC}"
}

run_container() {
    stop_container
    echo -e "${GREEN}Running container on HTTP ${HTTP_PORT}, gRPC ${GRPC_PORT}...${NC}"
    ENV_FILE_ARG=""
    if [ -f "$SCRIPT_DIR/.env" ]; then
        ENV_FILE_ARG="--env-file $SCRIPT_DIR/.env"
    fi
    docker run -d \
        -p "${HTTP_PORT}:8080" \
        -p "${GRPC_PORT}:50060" \
        --name "$CONTAINER_NAME" \
        $ENV_FILE_ARG \
        --rm \
        "$IMAGE_NAME"
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Container started.${NC}"
        echo -e "${BLUE}HTTP:     http://localhost:${HTTP_PORT}${NC}"
        echo -e "${BLUE}GraphQL:  http://localhost:${HTTP_PORT}/graphql${NC}"
        echo -e "${BLUE}gRPC:     localhost:${GRPC_PORT}${NC}"
    else
        echo -e "${RED}Failed to start container.${NC}"
        exit 1
    fi
}

case $choice in
    1)
        build_binary
        ;;
    2)
        run_local
        ;;
    3)
        build_image
        ;;
    4)
        run_container
        ;;
    5)
        build_image
        run_container
        ;;
    6)
        stop_container
        ;;
    7)
        echo -e "${GREEN}Cleaning build artifacts...${NC}"
        cd "$SCRIPT_DIR"
        cargo clean
        echo -e "${GREEN}Clean.${NC}"
        ;;
    8)
        echo "Exiting."
        exit 0
        ;;
    *)
        echo -e "${RED}Invalid option. Exiting.${NC}"
        exit 1
        ;;
esac
