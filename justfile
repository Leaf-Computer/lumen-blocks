# Run component development environment with bash (default)
dev-components:
    dx serve -p laminar-blocks --example main --platform web

dev-components-tailwind:
    cd blocks && tailwindcss -i tailwind.css -o assets/tailwind.css --config tailwind.config.js --watch
    
dev-docsite:
    dx serve -p docsite --platform web

dev-docsite-tailwind:
    cd docsite && tailwindcss -i tailwind.css -o assets/tailwind.css --config tailwind.config.js --watch

# Show available commands
default:
    @just --list
