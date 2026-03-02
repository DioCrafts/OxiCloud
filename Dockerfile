# syntax=docker/dockerfile:1
# ============================================================================
# Stage 1: PLANNER — Generate a dependency-only recipe from the full source
# ============================================================================
# cargo-chef inspects the real project structure (lib.rs + main.rs, features,
# build scripts, profile settings) and produces a minimal recipe.json that
# fingerprints ONLY dependency-relevant metadata. Source code changes that
# don't affect dependencies will NOT invalidate this layer.
FROM rust:1.93.0-alpine3.23 AS planner
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo chef prepare --recipe-path recipe.json

# ============================================================================
# Stage 2: COOK — Build all dependencies (cached until recipe.json changes)
# ============================================================================
# This stage compiles every dependency listed in recipe.json with the exact
# same profile, features, and target layout as the real build. Because it
# uses BuildKit cache mounts for the cargo registry and git checkouts,
# even a full rebuild after `docker system prune` only re-downloads crates
# that changed upstream — not the entire registry.
FROM rust:1.93.0-alpine3.23 AS cook
WORKDIR /app
RUN apk --no-cache upgrade && \
    apk add --no-cache musl-dev pkgconfig postgresql-dev gcc perl make
COPY --from=planner /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Cook dependencies only — no application source code is present.
# BuildKit cache mounts persist the cargo registry and target dir across
# builds so incremental recompilation works even for dependency updates.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json && \
    # Copy built artifacts out of the cache mount so the next stage can access them
    cp -r /app/target /app/target-out

# ============================================================================
# Stage 3: BUILD — Compile application source on top of pre-built deps
# ============================================================================
# Only this layer is invalidated when .rs files change. Dependencies are
# already compiled and linked from the cook stage.
FROM rust:1.93.0-alpine3.23 AS builder
WORKDIR /app
RUN apk --no-cache upgrade && \
    apk add --no-cache musl-dev pkgconfig postgresql-dev gcc perl make
# Bring in pre-compiled dependencies from cook
COPY --from=cook /app/target-out target
COPY --from=cook /usr/local/cargo/registry /usr/local/cargo/registry
# Copy project metadata and full source
COPY Cargo.toml Cargo.lock ./
COPY src src
COPY static static
COPY db db
# Build with all optimizations (DATABASE_URL only needed at compile-time for sqlx)
ARG DATABASE_URL="postgres://postgres:postgres@localhost/oxicloud"
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    DATABASE_URL="${DATABASE_URL}" cargo build --release

# ============================================================================
# Stage 4: RUNTIME — Minimal production image (~25 MB)
# ============================================================================
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
