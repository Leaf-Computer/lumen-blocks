# Dioxus Blocks Development Container

This directory contains configuration for a development container that provides a consistent environment for working on the Dioxus Blocks project.

## Features

The development container includes:

- **Rust Toolchain**: Latest stable Rust with commonly used components
  - rustfmt and clippy for code formatting and linting
  - Rust Analyzer integration for IDE support
  
- **Dioxus Tooling**:
  - Dioxus CLI (`dx`) pre-installed
  - All dependencies for web, desktop, and mobile development
  
- **VS Code Extensions**:
  - Rust Analyzer
  - Dioxus extension
  - TOML support
  - Crates.io integration
  - LLDB debugger

- **Development Tools**:
  - Git
  - GitHub CLI

## Usage

### VS Code

1. Install the [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension
2. Open this project in VS Code
3. Click on the green button in the bottom-left corner
4. Select "Reopen in Container"

### GitHub Codespaces

1. Navigate to the GitHub repository
2. Click on the "Code" button
3. Select the "Codespaces" tab
4. Click "Create codespace on main"

## Working with Dioxus in the Container

Once the container is running, you can:

- Run the example application:
  ```
  dx serve -p dioxus-blocks --example main --platform web
  ```

- Build the project:
  ```
  cargo build
  ```

- Run tests:
  ```
  cargo test
  ```

## Customizing the Container

If you need additional tools or settings:

1. Modify the `Dockerfile` to install additional dependencies
2. Update `devcontainer.json` to add more VS Code extensions or change settings
3. Rebuild the container by clicking the green button in VS Code and selecting "Rebuild Container"

## Troubleshooting

- **Permission issues**: The container runs as the `vscode` user. If you encounter permission problems, you may need to adjust file permissions.

- **Rust Analyzer issues**: If Rust Analyzer is not working properly, try restarting it from the command palette (Ctrl+Shift+P) by selecting "Rust Analyzer: Restart Server".

- **Container size**: The container includes a full Rust development environment and may be large. You can customize the `Dockerfile` to remove components you don't need.