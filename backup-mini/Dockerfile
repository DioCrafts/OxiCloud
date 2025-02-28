# Use an official Rust image as the base
FROM rust:1.85.0-slim

# Install pkg-config and OpenSSL development libraries
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Set the working directory
WORKDIR /app

# Configuración de variables de entorno para SQLx
ENV DATABASE_URL="postgres://myuser:mypassword@db/oxi_cloud_db"
# Si quieres compilar en modo offline, descomenta las siguientes líneas:
# COPY sqlx-data.json ./
# ENV SQLX_OFFLINE=true

# Copy Cargo manifest and source code
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY migrations/ migrations/
COPY templates/ templates/

# Build the application in release mode
RUN cargo build --release

# Expose the application's port (adjust as needed)
EXPOSE 8080

# Run the application
CMD ["./target/release/oxi_cloud"]

