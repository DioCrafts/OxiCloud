# Stage 1: Build the application
FROM rust:1.85-alpine AS builder
ARG alpine_mirror=""
ARG cargo_mirror=""
WORKDIR /app
RUN if test -n "${alpine_mirror}"; \
    then \
        echo "Using alpine mirror ${alpine_mirror}" &&\
        sed -i "s#https\?://dl-cdn.alpinelinux.org/alpine#${alpine_mirror}#g" /etc/apk/repositories; \
    fi && \
    if test -n "${cargo_mirror}"; \
    then \
        echo "Using cargo mirror ${cargo_mirror}" && mkdir -p .cargo && \
        echo -e "[source.crates-io]\nreplace-with = \"mirror\"\n[source.mirror]\nregistry=\"${cargo_mirror}\"" >> .cargo/config.toml; \
    fi && \
    apk --no-cache update && \
    apk --no-cache upgrade && \
    apk add --no-cache musl-dev openssl-dev pkgconfig postgresql-dev
COPY Cargo.toml Cargo.lock ./
# Create a minimal project to download and cache dependencies
RUN mkdir -p src && \
    echo 'fn main() { println!("Dummy build for caching dependencies"); }' > src/main.rs && \
    cargo build --release 

# Copy ALL files needed for compilation, including static files
COPY src src
COPY static static
COPY db db

# Build with all optimizations
RUN touch src/main.rs && \
    cargo build --release

# Stage 2: Create minimal final image
FROM alpine:3.21.3
ARG alpine_mirror
# Install only necessary runtime dependencies and update packages
RUN if test -n "${alpine_mirror}"; \
    then \
        echo "Using alpine mirror ${alpine_mirror}" &&\
        sed -i "s#https\?://dl-cdn.alpinelinux.org/alpine#${alpine_mirror}#g" /etc/apk/repositories; \
    fi && \
    apk --no-cache update && \
    apk --no-cache upgrade && \
    apk add --no-cache libgcc openssl ca-certificates libpq tzdata

# Copy only the compiled binary
COPY --from=builder /app/target/release/oxicloud /usr/local/bin/

# Copy static files and other resources needed at runtime
COPY static /app/static
COPY db /app/db

VOLUME ["/app/storage"]

# Set working directory
WORKDIR /app

# Run the application
CMD ["oxicloud"]
