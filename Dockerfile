# ─── Stage 1: Shared build base (avoids duplicate apk install) ────────────────
FROM rust:1.94.1-alpine3.23 AS base
RUN apk --no-cache upgrade && \
    apk add --no-cache musl-dev pkgconfig postgresql-dev gcc perl make

# ─── Stage 2: Cache dependencies ─────────────────────────────────────────────
FROM base AS cacher
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# build.rs + static/ are needed so the build script can run and set OUT_DIR
COPY build.rs ./
COPY static static
# Create a minimal project to download and cache dependencies
RUN mkdir -p src/bin && \
    echo 'fn main() { println!("Dummy build for caching dependencies"); }' > src/main.rs && \
    echo 'fn main() {}' > src/bin/generate-openapi.rs && \
    cargo build --release && \
    rm -rf src static-dist target/release/deps/oxicloud* target/release/build/oxicloud-*

# ─── Stage 3: Build the application ──────────────────────────────────────────
FROM base AS builder
WORKDIR /app
# Copy cached dependencies (only target dir and cargo registry)
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo/registry /usr/local/cargo/registry
# Copy source, build script, and static assets
COPY Cargo.toml Cargo.lock build.rs ./
COPY src src
COPY static static
COPY migrations migrations
# Build with all optimizations (DATABASE_URL only needed at compile-time for sqlx)
ARG DATABASE_URL="postgres://postgres:postgres@localhost/oxicloud"
RUN DATABASE_URL="${DATABASE_URL}" cargo build --release

# ─── Stage 4: Minimal runtime image ──────────────────────────────────────────
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
    apk add --no-cache libgcc ca-certificates libpq tzdata su-exec && \
    addgroup -g 1001 -S oxicloud && \
    adduser -u 1001 -S oxicloud -G oxicloud

# Copy the compiled binary and entrypoint (--chmod avoids extra RUN chmod layers)
COPY --from=builder --chmod=755 /app/target/release/oxicloud /usr/local/bin/
COPY --chmod=755 entrypoint.sh /usr/local/bin/entrypoint.sh

# Copy processed static files (bundled/minified by build.rs in release)
COPY --from=builder --chown=oxicloud:oxicloud /app/static-dist /app/static
# Create storage directory with proper permissions
RUN mkdir -p /app/storage && chown -R oxicloud:oxicloud /app/storage

# Set working directory
WORKDIR /app

# Expose application port
EXPOSE 8086

# Basic health check — verifies the HTTP server responds on the main port.
# Docker / Compose / Swarm will mark the container unhealthy after 3 failures.
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget -qO- http://localhost:8086/api/version || exit 1

# Entrypoint fixes volume permissions then drops to oxicloud user.
# The container starts as root so it can chown mounted volumes,
# then su-exec drops privileges before running the application.
ENTRYPOINT ["entrypoint.sh"]
CMD ["oxicloud"]
