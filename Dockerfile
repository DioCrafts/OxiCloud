# Stage 1: Cache dependencies
FROM rust:1.93.0-alpine3.23 AS cacher
WORKDIR /app
RUN apk --no-cache upgrade && \
    apk add --no-cache musl-dev pkgconfig postgresql-dev gcc perl make
COPY Cargo.toml Cargo.lock ./
# Create a minimal project to download and cache dependencies
RUN mkdir -p src && \
    echo 'fn main() { println!("Dummy build for caching dependencies"); }' > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/oxicloud*
# Stage 2: Build the application
FROM rust:1.93.0-alpine3.23 AS builder
WORKDIR /app
RUN apk --no-cache upgrade && \
    apk add --no-cache musl-dev pkgconfig postgresql-dev gcc perl make
# Copy cached dependencies (only target dir and cargo registry)
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo/registry /usr/local/cargo/registry
# Copy source and static (login.html is embedded at compile-time via include_str!)
COPY Cargo.toml Cargo.lock ./
COPY src src
COPY static static
# Build with all optimizations (DATABASE_URL only needed at compile-time for sqlx)
ARG DATABASE_URL="postgres://postgres:postgres@localhost/oxicloud"
RUN DATABASE_URL="${DATABASE_URL}" cargo build --release

# Stage 3: Create minimal final image
FROM alpine:3.23.3

# OCI image metadata
LABEL org.opencontainers.image.title="OxiCloud" \
      org.opencontainers.image.description="Ultra-fast, secure & lightweight self-hosted cloud storage built in Rust" \
      org.opencontainers.image.url="https://github.com/DioCrafts/OxiCloud" \
      org.opencontainers.image.source="https://github.com/DioCrafts/OxiCloud" \
      org.opencontainers.image.vendor="DioCrafts" \
      org.opencontainers.image.licenses="MIT"

# Install only necessary runtime dependencies and update packages
# su-exec is needed by the entrypoint to drop privileges after fixing volume permissions
RUN apk --no-cache upgrade && \
    apk add --no-cache libgcc ca-certificates libpq tzdata su-exec

# Create non-root user
RUN addgroup -g 1001 -S oxicloud && \
    adduser -u 1001 -S oxicloud -G oxicloud

# Copy only the compiled binary
COPY --from=builder /app/target/release/oxicloud /usr/local/bin/
RUN chmod +x /usr/local/bin/oxicloud

# Copy entrypoint script
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Copy static files and other resources needed at runtime
COPY --chown=oxicloud:oxicloud static /app/static
COPY --chown=oxicloud:oxicloud db /app/db

# Create storage directory with proper permissions
RUN mkdir -p /app/storage && chown -R oxicloud:oxicloud /app/storage

# Set working directory
WORKDIR /app

# Expose application port
EXPOSE 8086

# Entrypoint fixes volume permissions then drops to oxicloud user.
# The container starts as root so it can chown mounted volumes,
# then su-exec drops privileges before running the application.
ENTRYPOINT ["entrypoint.sh"]
CMD ["oxicloud"]
