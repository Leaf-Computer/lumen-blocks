# Installation

This guide will walk you through the process of installing and setting up Laminar Blocks in your Dioxus project.

## Prerequisites

Before installing Laminar Blocks, ensure you have:

- [Rust](https://www.rust-lang.org/tools/install) installed
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/#installing-the-cli) installed

## Setting Up a Dioxus Project with Tailwind

Please refer to the official Dioxus docs to find out how to setup a project with Tailwind: [Dioxus Tailwind guide](https://dioxuslabs.com/learn/0.6/cookbook/tailwind)

## Installing Laminar Blocks

You have two options for installing Laminar Blocks:

### Option 1: Add as a Dependency

1. Add Laminar Blocks to your `Cargo.toml`:

```toml
[dependencies]
dioxus = "0.6.0"
dioxus-web = "0.6.0"
laminar-blocks = "0.1.0"
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

3. Update your `tailwind.config.js` to include the Laminar Blocks components:

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    // Include Laminar Blocks components
    "${process.env.HOME}/.cargo/registry/src/**/laminar-blocks-*/src/**/*.{rs,html,css}"
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
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

**Note:** Please ensure to use Tailwind v3. Tailwind v4 support is coming soon!

### Option 2: Add Locally for Modifications

1. Create a workspace structure for your project:

```
my-project/
├── Cargo.toml         # Workspace manifest
├── app/               # Your application
│   └── Cargo.toml    # App manifest
└── laminar-blocks/    # Local copy of the laminar-blocks crate
    └── ...
```

2. Set up your workspace `Cargo.toml`:

```toml
[workspace]
members = [
    "app",
    "laminar-blocks"
]

[workspace.dependencies]
laminar-blocks = { path = "./laminar-blocks" }
```

3. Clone Laminar Blocks into your workspace:

```bash
git clone https://github.com/Leaf-Computer/laminar-blocks.git
```

4. Update your app's `Cargo.toml` to reference the local copy:

```toml
[dependencies]
dioxus = "0.6.0"
dioxus-web = "0.6.0"
laminar-blocks.workspace = true
```

5. Create a `tailwind.css` file in your app directory (same as in Option 1).

6. Update your `tailwind.config.js` to include the local Laminar Blocks components:

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    // Include local Laminar Blocks components
    "../laminar-blocks/**/*.{rs,html,css}"
  ],
  // Rest of the configuration same as Option 1
  // ...
};
```

7. Generate the Tailwind CSS output:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

## Using Laminar Blocks

After installation, you can import and use Laminar Blocks components in your Dioxus application:

```rust
use dioxus::prelude::*;
use laminar_blocks::button::Button;

fn App() -> Element {
    rsx! {
        div {
            h1 { "Hello, Laminar Blocks!" }
            Button {
                variant: "default",
                onclick: move |_| {
                    println!("Button clicked!");
                },
                "Click Me"
            }
        }
    }
}
```

## Troubleshooting

If Tailwind CSS classes aren't being applied:

1. Ensure your `tailwind.config.js` correctly points to both your source files and the Laminar Blocks components.
2. Make sure you're generating the Tailwind CSS output file and including it in your application.
3. Check for any path errors in the `content` array of your Tailwind configuration.

If components aren't rendering correctly:

1. Verify you've imported the components correctly.
2. Ensure you're using the latest compatible versions of Dioxus and Laminar Blocks.
