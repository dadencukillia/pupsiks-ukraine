# Stage 1: Build the application
FROM rust:1.91-alpine AS builder
WORKDIR /app
RUN apk add openssl-dev openssl-libs-static pkgconfig
COPY . .
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --bin backend && \
    cp target/release/backend ./backend

# Stage 2: Create the final runtime image
FROM alpine:3.22 AS runtime
COPY --from=builder /app/backend /app/server
EXPOSE 8080
CMD ["/app/server"]
