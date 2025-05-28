FROM mcr.microsoft.com/vscode/devcontainers/rust:latest

# Install dependencies for Dioxus
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    xdg-utils \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-binstall for faster installation of Rust binaries
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install Dioxus CLI
RUN cargo binstall -y dioxus-cli

# Add cargo bin to PATH
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Install additional Rust components
RUN rustup component add rustfmt clippy

# Set up for better development experience
ENV RUST_BACKTRACE=1

# Create a non-root user with a home directory
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Ensure ~/.cargo/bin is in PATH
RUN echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> /home/$USERNAME/.bashrc

# Set working directory
WORKDIR /workspaces/app

# Default command
CMD ["bash"]