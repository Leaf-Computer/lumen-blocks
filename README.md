<div align="center">
  <h1>🧱 Dioxus Blocks</h1>
  <p><strong>Accessible, styled, opinion components for Dioxus.</strong></p>
</div>

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/dioxus-primitives">
    <img src="https://img.shields.io/crates/v/dioxus-primitives.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dioxus-primitives">
    <img src="https://img.shields.io/crates/d/dioxus-primitives.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/dioxus-primitives">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

-----
<br/>

Dioxus primitives is an ARIA-accessible, styled, opinionated component library for Dioxus based on the shadcn project, and built on the Dioxus Primitives unstyled components library.


## Here's what we have.
We're still in the early days - Many components are still being created and stabilized.

23/28
- [ ] Button - In Progress

## Running the preview.

You can preview the components with:
```
dx serve -p dioxus-blocks --example main --platform web
```

You should run this during development to keep tailwind classes up to date

```
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --config tailwind.config.js --watch
```

## Development Container

This project includes a devcontainer configuration for Visual Studio Code and GitHub Codespaces. The devcontainer provides a consistent development environment with all the necessary tools for Rust and Dioxus development pre-installed.

### Features

- Rust toolchain with Rust Analyzer
- Dioxus CLI (`dx`) pre-installed
- Required dependencies for desktop/web/mobile development
- Helpful VS Code extensions for Rust development

### Usage

#### With VS Code:

1. Install the [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension
2. Open the project in VS Code
3. Click on the green button in the bottom-left corner and select "Reopen in Container"

#### With GitHub Codespaces:

1. Navigate to the GitHub repository
2. Click on the "Code" button and select the "Codespaces" tab
3. Click "Create codespace on main"

The container will automatically install all required dependencies and tools. Once the container is ready, you can start developing with full IDE support.

## License
This project is licensed under the [MIT](./LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this repository, by you, shall be licensed as MIT, without any additional terms or conditions.
