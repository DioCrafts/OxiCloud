set dotenv-load

default:
    @just --list

build:
    cargo build

release:
    cargo build --release

run:
    cargo run

run-debug:
    RUST_LOG=debug cargo run

test:
    cargo test --workspace

test-mocks:
    cargo test --features test_utils

test-one name:
    cargo test {{name}}

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

lint:
    cargo clippy -- -D warnings

check:
    cargo fmt --all
    cargo clippy -- -D warnings

openapi:
    cargo run --bin generate-openapi

db:
    docker compose up -d postgres

db-down:
    docker compose down
