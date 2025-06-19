dev-components-tailwind:
    cd blocks && tailwindcss -i tailwind.css -o assets/tailwind.css --config tailwind.config.js --watch
    
dev-docsite:
    dx serve -p docsite --platform web --port 8081

dev-docsite-tailwind:
    cd docsite && tailwindcss -i tailwind.css -o assets/tailwind.css --config tailwind.config.js --watch
    
build-docs:
    cd docsite/docs && cargo build

# Show available commands
default:
    @just --list
