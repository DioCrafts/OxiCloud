FROM rust:1.85 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir benches
COPY benches/file_operations.rs ./benches/

# Add dummy project to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .

RUN cargo build --release

FROM alpine:latest as release
WORKDIR /app


# Copy the binaries from the builder stage
COPY --from=builder /app/target/release/migrate /app/migrate
COPY --from=builder /app/target/release/oxicloud /app/oxicloud

# First always run migrations, then app
# Can be improved by adding a dedicated entrypoint.sh script
# but that is probably not needed for now
CMD ["/bin/sh", "-c", "/app/migrate && /app/oxicloud"]

