# Stage 1: Build the application
FROM rust:1.91-slim-bullseye AS builder
WORKDIR /app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config && \
    rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -rf src
COPY . .
RUN cargo build --release --bin backend

# Stage 2: Create the final runtime image
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl1.1 && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend /usr/local/bin/
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/backend"]
