# Stage 1: Build the application
FROM rust:1.91 AS builder
WORKDIR /app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config && \
    rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release --bin backend

# Stage 2: Create the final runtime image
FROM gcr.io/distroless/cc-debian12 AS runtime
COPY --from=builder /app/target/release/backend /app/server
EXPOSE 8080
ENTRYPOINT ["/app/server"]
