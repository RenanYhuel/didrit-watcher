FROM rust:bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/didrit-watcher /app/didrit-watcher

RUN mkdir -p /app/data

ENV STATE_FILE_PATH=/app/data/state.json

VOLUME ["/app/data"]

ENTRYPOINT ["/app/didrit-watcher"]
