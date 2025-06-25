dev-docsite:
    dx serve -p docsite --platform web --port 8081

dev-docsite-tailwind:
    cd docsite && tailwindcss -i tailwind.css -o assets/tailwind.css --config tailwind.config.js --watch
    
build-docs:
    cd docsite/docs && cargo build

pre-commit:
    cargo fmt --all
    cd docsite && tailwindcss -i tailwind.css -o assets/tailwind.css --config tailwind.config.js

build-docsite:
    cd docsite && tailwindcss -i tailwind.css -o assets/tailwind.css --config tailwind.config.js
    dx bundle -p docsite --platform web --features analytics --release
    cp docsite/assets/_redirects target/dx/docsite/release/web/public

# Show available commands
default:
    @just --list
