# Run component development environment with bash (default)
dev-components:
    dx serve -p dioxus-blocks --example main --platform web

dev-tailwind-watch:
    tailwindcss -i blocks/tailwind.css -o blocks/assets/tailwind.css --config blocks/tailwind.config.js --watch

# Show available commands
default:
    @just --list
