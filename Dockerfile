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
# Copy cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# Copy ALL files needed for compilation, including static files
COPY src src
COPY static static
COPY db db
COPY Cargo.toml Cargo.lock ./
# Build with all optimizations
ENV DATABASE_URL="postgres://postgres:postgres@postgres/oxicloud"
RUN cargo build --release

# Stage 3: Create minimal final image
FROM alpine:3.23.3
# Install only necessary runtime dependencies and update packages
RUN apk --no-cache upgrade && \
    apk add --no-cache libgcc ca-certificates libpq tzdata

# Copy only the compiled binary
COPY --from=builder /app/target/release/oxicloud /usr/local/bin/

# Copy static files and other resources needed at runtime
COPY static /app/static
COPY db /app/db

# Create storage directory with proper permissions
RUN mkdir -p /app/storage && chmod 777 /app/storage

# Set proper permissions
RUN chmod +x /usr/local/bin/oxicloud

# Set working directory
WORKDIR /app

# Run the application
CMD ["oxicloud"]
