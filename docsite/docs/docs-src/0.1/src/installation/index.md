# Installation

This guide will walk you through the process of installing and setting up Lumen Blocks (version `v0.1.0`) in your Dioxus project.

## Prerequisites

Before installing Lumen Blocks, ensure you have:

- [Rust](https://www.rust-lang.org/tools/install) installed
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/#installing-the-cli) installed

## Starting from our base project template (new projects)

1. Clone the starter template:

```bash
git clone https://github.com/Leaf-Computer/lumen-blocks-starter.git my-lumen-app
```

2. Navigate to the newly created project directory:

```bash
cd my-lumen-app
```

3. Install dependencies and build the project:

```bash
cargo build
```

4. Start the development server:

```bash
dx serve
```

5. Open your browser and visit `http://localhost:8080` to see your new Lumen Blocks project.

6. For customization options, component documentation, and best practices, refer to the `README.md` file included in the project.


## Installing Lumen Blocks on an existing project

These options assume you have an existing Dioxus 0.6 project setup with Tailwind, similar to the one in the [Dioxus Tailwind guide](https://dioxuslabs.com/learn/0.6/cookbook/tailwind).

For an existing project, you have two options for installing Lumen Blocks:

### Option 1: Add as a Dependency

1. Add Lumen Blocks to your `Cargo.toml`:

```toml
[dependencies]
dioxus = "0.6.0"
dioxus-web = "0.6.0"
# Add this line
lumen-blocks = { git = "https://github.com/Leaf-Computer/lumen-blocks.git", rev = "v0.1.0" }
```

2. Create a `tailwind.css` file in your project's root or assets directory, with the following content:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

body {
    background-color: rgb(var(--background));
    color: rgb(var(--foreground));
}

:root {
--radius: 0.75rem;
--background: 255 255 255;
--foreground: 10 10 10;
--card: 255 255 255;
--card-foreground: 10 10 10;
--popover: 255 255 255;
--popover-foreground: 10 10 10;
--primary: 23 23 23;
--primary-foreground: 250 250 250;
--secondary: 245 245 245;
--secondary-foreground: 23 23 23;
--muted: 245 245 245;
--muted-foreground: 115 115 115;
--accent: 245 245 245;
--accent-foreground: 23 23 23;
--destructive: 227 0 17;
--border: 229 229 229;
--input: 229 229 229;
--ring: 161 161 161;
--chart-1: 223 97 0;
--chart-2: 0 149 136;
--chart-3: 16 78 100;
--chart-4: 249 188 0;
--chart-5: 242 159 0;
--sidebar: 250 250 250;
--sidebar-foreground: 10 10 10;
--sidebar-primary: 23 23 23;
--sidebar-primary-foreground: 250 250 250;
--sidebar-accent: 245 245 245;
--sidebar-accent-foreground: 23 23 23;
--sidebar-border: 229 229 229;
--sidebar-ring: 161 161 161;
}

@media (prefers-color-scheme: dark) {
    :root {
    --background: 10 10 10;
    --foreground: 250 250 250;
    --card: 23 23 23;
    --card-foreground: 250 250 250;
    --popover: 10 10 10;
    --popover-foreground: 250 250 250;
    --primary: 229 229 229;
    --primary-foreground: 23 23 23;
    --secondary: 38 38 38;
    --secondary-foreground: 250 250 250;
    --muted: 38 38 38;
    --muted-foreground: 161 161 161;
    --accent: 64 64 64;
    --accent-foreground: 250 250 250;
    --destructive: 191 0 13;
    --border: 255 255 255 / 20%;
    --input: 255 255 255 / 15%;
    --ring: 115 115 115;
    --chart-1: 20 71 230;
    --chart-2: 0 186 129;
    --chart-3: 242 159 0;
    --chart-4: 171 75 255;
    --chart-5: 255 35 87;
    --sidebar: 23 23 23;
    --sidebar-foreground: 250 250 250;
    --sidebar-primary: 20 71 230;
    --sidebar-primary-foreground: 250 250 250;
    --sidebar-accent: 38 38 38;
    --sidebar-accent-foreground: 250 250 250;
    --sidebar-border: 255 255 255 / 10%;
    --sidebar-ring: 82 82 82;
    }
}
```

3. Update your `tailwind.config.js` to include the Lumen Blocks components:

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // Include your source files
    "./src/**/*.{rs,html,css}",
    // Include Lumen Blocks components
    // Note: The `9beadef` on the path is there to match version v0.1.0. If you update Lumen Blocks on your project, you should update this path as well with the first 7 digits of the commit hash.
    `${process.env.HOME}/.cargo/git/checkouts/lumen-blocks-*/9beadef/blocks/src/**/*.rs`
  ],
  theme: {
    extend: {
      colors: {
        border: "rgb(var(--border))",
        input: "rgb(var(--input))",
        ring: "rgb(var(--ring))",
        background: "rgb(var(--background))",
        foreground: "rgb(var(--foreground))",
        primary: {
          DEFAULT: "rgb(var(--primary))",
          foreground: "rgb(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "rgb(var(--secondary))",
          foreground: "rgb(var(--secondary-foreground))",
        },
        destructive: {
          DEFAULT: "rgb(var(--destructive))",
          foreground: "rgb(var(--primary-foreground))",
        },
        muted: {
          DEFAULT: "rgb(var(--muted))",
          foreground: "rgb(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "rgb(var(--accent))",
          foreground: "rgb(var(--accent-foreground))",
        },
        popover: {
          DEFAULT: "rgb(var(--popover))",
          foreground: "rgb(var(--popover-foreground))",
        },
        card: {
          DEFAULT: "rgb(var(--card))",
          foreground: "rgb(var(--card-foreground))",
        },
        sidebar: {
          DEFAULT: "rgb(var(--sidebar))",
          foreground: "rgb(var(--sidebar-foreground))",
          primary: "rgb(var(--sidebar-primary))",
          "primary-foreground": "rgb(var(--sidebar-primary-foreground))",
          accent: "rgb(var(--sidebar-accent))",
          "accent-foreground": "rgb(var(--sidebar-accent-foreground))",
          border: "rgb(var(--sidebar-border))",
          ring: "rgb(var(--sidebar-ring))",
        },
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
      keyframes: {
        "accordion-down": {
          from: { height: 0 },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: 0 },
        },
        "slide-in-from-right": {
          from: { transform: "translateX(100%)" },
          to: { transform: "translateX(0)" },
        },
        "slide-out-to-right": {
          from: { transform: "translateX(0)" },
          to: { transform: "translateX(100%)" },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
        "slide-in-from-right": "slide-in-from-right 0.2s ease-out",
        "slide-out-to-right": "slide-out-to-right 0.2s ease-out",
      },
    },
  },
  plugins: [
    function({ addUtilities }) {
      addUtilities({
        ".animate-in": {
          "animation-fill-mode": "forwards",
          "animation-timing-function": "cubic-bezier(0.16, 1, 0.3, 1)",
        },
        ".animate-out": {
          "animation-fill-mode": "forwards",
          "animation-timing-function": "cubic-bezier(0.16, 1, 0.3, 1)",
        },
        ".slide-in-from-right": {
          "animation-name": "slide-in-from-right",
        },
        ".slide-out-to-right": {
          "animation-name": "slide-out-to-right",
        },
      });
    },
  ],
};
```

4. Generate the Tailwind CSS output:

```bash
npx tailwindcss@3 -i ./tailwind.css -o ./assets/tailwind.css --watch
```

**Note:** This project currently only supports Tailwind v3. Tailwind v4 support is coming [in the future](https://github.com/Leaf-Computer/lumen-blocks/issues/24)!

### Option 2: Add Locally for component customization

1. Create a workspace structure for your project:

```
my-project/
├── Cargo.toml         # Workspace manifest
├── app/               # Your application
│   └── Cargo.toml    # App manifest
└── lumen-blocks/      # Local copy of the lumen-blocks repository (Copied or via git submodules)
    └── blocks         # blocks crate
```

You can manually get the repository by running this command inside your workspace:

```bash
git clone https://github.com/Leaf-Computer/lumen-blocks.git
```

2. Set up your workspace `Cargo.toml`:

```toml
[workspace]
members = [
    "app",
    "lumen-blocks"
]

[workspace.dependencies]
lumen-blocks = { path = "./lumen-blocks/blocks" }
```

3. Update your app's `Cargo.toml` to reference the local copy:

```toml
[dependencies]
dioxus = "0.6.0"
dioxus-web = "0.6.0"
lumen-blocks.workspace = true
```

4. Create a `tailwind.css` file in your app directory (same as in Option 1).

5. Update your `tailwind.config.js` in your app folder to include the local Lumen Blocks components:

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    // Include local Lumen Blocks components
    "../lumen-blocks/blocks/**/*.rs"
  ],
  theme: {
    extend: {
      colors: {
        border: "rgb(var(--border))",
        input: "rgb(var(--input))",
        ring: "rgb(var(--ring))",
        background: "rgb(var(--background))",
        foreground: "rgb(var(--foreground))",
        primary: {
          DEFAULT: "rgb(var(--primary))",
          foreground: "rgb(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "rgb(var(--secondary))",
          foreground: "rgb(var(--secondary-foreground))",
        },
        destructive: {
          DEFAULT: "rgb(var(--destructive))",
          foreground: "rgb(var(--primary-foreground))",
        },
        muted: {
          DEFAULT: "rgb(var(--muted))",
          foreground: "rgb(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "rgb(var(--accent))",
          foreground: "rgb(var(--accent-foreground))",
        },
        popover: {
          DEFAULT: "rgb(var(--popover))",
          foreground: "rgb(var(--popover-foreground))",
        },
        card: {
          DEFAULT: "rgb(var(--card))",
          foreground: "rgb(var(--card-foreground))",
        },
        sidebar: {
          DEFAULT: "rgb(var(--sidebar))",
          foreground: "rgb(var(--sidebar-foreground))",
          primary: "rgb(var(--sidebar-primary))",
          "primary-foreground": "rgb(var(--sidebar-primary-foreground))",
          accent: "rgb(var(--sidebar-accent))",
          "accent-foreground": "rgb(var(--sidebar-accent-foreground))",
          border: "rgb(var(--sidebar-border))",
          ring: "rgb(var(--sidebar-ring))",
        },
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
      keyframes: {
        "accordion-down": {
          from: { height: 0 },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: 0 },
        },
        "slide-in-from-right": {
          from: { transform: "translateX(100%)" },
          to: { transform: "translateX(0)" },
        },
        "slide-out-to-right": {
          from: { transform: "translateX(0)" },
          to: { transform: "translateX(100%)" },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
        "slide-in-from-right": "slide-in-from-right 0.2s ease-out",
        "slide-out-to-right": "slide-out-to-right 0.2s ease-out",
      },
    },
  },
  plugins: [
    function({ addUtilities }) {
      addUtilities({
        ".animate-in": {
          "animation-fill-mode": "forwards",
          "animation-timing-function": "cubic-bezier(0.16, 1, 0.3, 1)",
        },
        ".animate-out": {
          "animation-fill-mode": "forwards",
          "animation-timing-function": "cubic-bezier(0.16, 1, 0.3, 1)",
        },
        ".slide-in-from-right": {
          "animation-name": "slide-in-from-right",
        },
        ".slide-out-to-right": {
          "animation-name": "slide-out-to-right",
        },
      });
    },
  ],
};
```

6. Generate the Tailwind CSS output:

```bash
npx tailwindcss@3 -i ./tailwind.css -o ./assets/tailwind.css --watch
```

## Using Lumen Blocks

After installation, you can import and use Lumen Blocks components in your Dioxus application. Please see the other documentation pages for examples!

## Troubleshooting

If Tailwind CSS classes aren't being applied:

- Ensure your `tailwind.config.js` correctly points to both your source files and the Lumen Blocks components.
- Make sure you're generating the Tailwind CSS output file and including it in your application.
- Check for any path errors in the `content` array of your Tailwind configuration.

If components aren't rendering correctly:

- Verify you've imported the components correctly.
- Ensure you're using the latest compatible versions of Dioxus and Lumen Blocks.

If you are still running into issues, please open an issue here: [Issues](https://github.com/Leaf-Computer/lumen-blocks/issues)
