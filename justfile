# Run component development environment with bash (default)
dev-components:
    dx serve -p laminar-blocks --example main --platform web

dev-components-tailwind:
    cd blocks && tailwindcss -i tailwind.css -o assets/tailwind.css --config tailwind.config.js --watch
    
dev-sample:
    dx serve -p sample-app --platform web

dev-sample-tailwind:
    cd sample-app && tailwindcss -i tailwind.css -o assets/tailwind.css --config tailwind.config.js --watch

# Show available commands
default:
    @just --list
