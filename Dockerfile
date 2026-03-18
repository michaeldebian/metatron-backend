# ── Stage 1: Build ────────────────────────────────────────────────────────────
FROM rust:1.94-alpine AS builder

RUN apk add --no-cache musl-dev protobuf-dev protoc

WORKDIR /build

# Copy shared protos (build context must be set to orchestrator root)
COPY proto/ /build/proto/

# Copy backend source
COPY metatron-backend/ /build/metatron-backend/

WORKDIR /build/metatron-backend

# Build release binary
RUN cargo build --release --bin metatron-api

# ── Stage 2: Runtime ─────────────────────────────────────────────────────────
FROM alpine:3.21

RUN apk add --no-cache ca-certificates

RUN adduser -D metatron
USER metatron

COPY --from=builder /build/metatron-backend/target/release/metatron-api /usr/local/bin/metatron-api

EXPOSE 8080 50060

ENTRYPOINT ["metatron-api"]
