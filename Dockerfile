# Stage 1: Cache dependencies
FROM rust:1.85-alpine AS cacher
WORKDIR /app

# Instalar dependencias de compilación optimizadas
RUN apk --no-cache upgrade && \
    apk add --no-cache musl-dev pkgconfig postgresql-dev gcc perl make

# Configuración para compilación optimizada
ENV RUSTFLAGS="-C target-cpu=native"
ENV CARGO_NET_RETRY=10

# Copiar solo los archivos de dependencias
COPY Cargo.toml Cargo.lock ./

# Crear un proyecto mínimo para descargar y cachear dependencias
RUN mkdir -p src && \
    echo 'fn main() { println!("Dummy build for caching dependencies"); }' > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/oxicloud*

# Stage 2: Build the application
FROM rust:1.85-alpine AS builder
WORKDIR /app

# Instalar dependencias de compilación
RUN apk --no-cache upgrade && \
    apk add --no-cache musl-dev pkgconfig postgresql-dev gcc perl make

# Configuración para compilación optimizada
ENV RUSTFLAGS="-C target-cpu=native"
ENV CARGO_NET_RETRY=10

# Copiar dependencias en caché
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Copiar archivos de código fuente primero (más propenso a cambiar)
COPY src src
COPY Cargo.toml Cargo.lock ./

# Copiar recursos estáticos (menos propensos a cambiar)
COPY static static
COPY db db

# Configuración de compilación y compilar con todas las optimizaciones
ARG VERSION=dev
ENV VERSION=${VERSION}
ENV DATABASE_URL="postgres://postgres:postgres@postgres/oxicloud"

# Compilar con optimizaciones de release
RUN cargo build --release

# Stage 3: Create minimal final image
FROM alpine:3.21.3 AS runtime

# Metadatos de la imagen
LABEL org.opencontainers.image.title="OxiCloud"
LABEL org.opencontainers.image.description="☁️ OxiCloud server, efficient and secure way to save all your data"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/DioCrafts/OxiCloud"
ARG VERSION=dev
LABEL org.opencontainers.image.version=${VERSION}

# Instalar solo dependencias de tiempo de ejecución necesarias y actualizar paquetes
RUN apk --no-cache upgrade && \
    apk add --no-cache libgcc ca-certificates libpq tzdata

# Copiar solo el binario compilado
COPY --from=builder /app/target/release/oxicloud /usr/local/bin/

# Copiar archivos estáticos y otros recursos necesarios en tiempo de ejecución
COPY static /app/static
COPY db /app/db

# Crear directorio de almacenamiento con permisos adecuados
RUN mkdir -p /app/storage && chmod 777 /app/storage

# Establecer permisos adecuados
RUN chmod +x /usr/local/bin/oxicloud

# Establecer directorio de trabajo
WORKDIR /app

# Puerto de la aplicación
EXPOSE 8086

# Ejecutar la aplicación
CMD ["oxicloud"]