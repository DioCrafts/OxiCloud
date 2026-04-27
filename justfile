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
    cargo clippy --all-features --all-targets -- -D warnings

check:
    cargo fmt --all
    cargo clippy --all-features --all-targets -- -D warnings

openapi:
    cargo run --bin generate-openapi

db:
    docker compose up -d postgres

db-down:
    docker compose down

# start OxiCloud with static assets in no-cache mode
front-dev:
    PROFILE=dev cargo run

front-fmt:
    biome format static/

front-lint:
    biome lint static/

# check CSS rules
front-rules:
    stylelint static/css/
