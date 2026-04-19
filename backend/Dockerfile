# Stage 1: Build
FROM rust:1.83-slim AS builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build the backend
RUN cargo build --release -p backend

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/backend .
COPY --from=builder /usr/src/app/target/release/verify_audit .


EXPOSE 8080

CMD ["./backend"]
