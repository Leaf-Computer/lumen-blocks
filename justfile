dev-docsite:
    dx serve -p docsite --web --port 8081
    
build-docs:
    cd docsite/docs && cargo build

pre-commit:
    cargo fmt --all
    dx build --package docsite

build-docsite:
    dx bundle -p docsite --web --features analytics --release
    cp docsite/assets/_redirects target/dx/docsite/release/web/public

# Show available commands
default:
    @just --list
