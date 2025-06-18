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

2. Create a `tailwind.css` file in your project's root or assets directory:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

body {
    background-color: var(--background);
    color: var(--foreground);
}

:root {
--radius: 0.75rem;
--background: oklch(1 0 0);
--foreground: oklch(0.145 0 0);
--card: oklch(1 0 0);
--card-foreground: oklch(0.145 0 0);
--popover: oklch(1 0 0);
--popover-foreground: oklch(0.145 0 0);
--primary: oklch(0.205 0 0);
--primary-foreground: oklch(0.985 0 0);
--secondary: oklch(0.97 0 0);
--secondary-foreground: oklch(0.205 0 0);
--muted: oklch(0.97 0 0);
--muted-foreground: oklch(0.556 0 0);
--accent: oklch(0.97 0 0);
--accent-foreground: oklch(0.205 0 0);
--destructive: oklch(0.577 0.245 27.325);
--border: oklch(0.922 0 0);
--input: oklch(0.922 0 0);
--ring: oklch(0.708 0 0);
--chart-1: oklch(0.646 0.222 41.116);
--chart-2: oklch(0.6 0.118 184.704);
--chart-3: oklch(0.398 0.07 227.392);
--chart-4: oklch(0.828 0.189 84.429);
--chart-5: oklch(0.769 0.188 70.08);
--sidebar: oklch(0.985 0 0);
--sidebar-foreground: oklch(0.145 0 0);
--sidebar-primary: oklch(0.205 0 0);
--sidebar-primary-foreground: oklch(0.985 0 0);
--sidebar-accent: oklch(0.97 0 0);
--sidebar-accent-foreground: oklch(0.205 0 0);
--sidebar-border: oklch(0.922 0 0);
--sidebar-ring: oklch(0.708 0 0);
}

@media (prefers-color-scheme: dark) {
    :root {
    --background: oklch(0.145 0 0);
    --foreground: oklch(0.985 0 0);
    --card: oklch(0.205 0 0);
    --card-foreground: oklch(0.985 0 0);
    --popover: oklch(0.145 0 0);
    --popover-foreground: oklch(0.985 0 0);
    --primary: oklch(0.922 0 0);
    --primary-foreground: oklch(0.205 0 0);
    --secondary: oklch(0.269 0 0);
    --secondary-foreground: oklch(0.985 0 0);
    --muted: oklch(0.269 0 0);
    --muted-foreground: oklch(0.708 0 0);
    --accent: oklch(0.371 0 0);
    --accent-foreground: oklch(0.985 0 0);
    --destructive: oklch(0.628 0.2577 29.23 / 40%);
    --border: oklch(1 0 0 / 20%);
    --input: oklch(1 0 0 / 15%);
    --ring: oklch(0.556 0 0);
    --chart-1: oklch(0.488 0.243 264.376);
    --chart-2: oklch(0.696 0.17 162.48);
    --chart-3: oklch(0.769 0.188 70.08);
    --chart-4: oklch(0.627 0.265 303.9);
    --chart-5: oklch(0.645 0.246 16.439);
    --sidebar: oklch(0.205 0 0);
    --sidebar-foreground: oklch(0.985 0 0);
    --sidebar-primary: oklch(0.488 0.243 264.376);
    --sidebar-primary-foreground: oklch(0.985 0 0);
    --sidebar-accent: oklch(0.269 0 0);
    --sidebar-accent-foreground: oklch(0.985 0 0);
    --sidebar-border: oklch(1 0 0 / 10%);
    --sidebar-ring: oklch(0.439 0 0);
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
        border: "var(--border)",
        input: "var(--input)",
        ring: "var(--ring)",
        background: "var(--background)",
        foreground: "var(--foreground)",
        primary: {
          DEFAULT: "var(--primary)",
          foreground: "var(--primary-foreground)",
        },
        secondary: {
          DEFAULT: "var(--secondary)",
          foreground: "var(--secondary-foreground)",
        },
        destructive: {
          DEFAULT: "var(--destructive)",
          foreground: "var(--primary-foreground)",
        },
        muted: {
          DEFAULT: "var(--muted)",
          foreground: "var(--muted-foreground)",
        },
        accent: {
          DEFAULT: "var(--accent)",
          foreground: "var(--accent-foreground)",
        },
        popover: {
          DEFAULT: "var(--popover)",
          foreground: "var(--popover-foreground)",
        },
        card: {
          DEFAULT: "var(--card)",
          foreground: "var(--card-foreground)",
        },
        sidebar: {
          DEFAULT: "var(--sidebar)",
          foreground: "var(--sidebar-foreground)",
          primary: "var(--sidebar-primary)",
          "primary-foreground": "var(--sidebar-primary-foreground)",
          accent: "var(--sidebar-accent)",
          "accent-foreground": "var(--sidebar-accent-foreground)",
          border: "var(--sidebar-border)",
          ring: "var(--sidebar-ring)",
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

### Option 2: Add Locally for Modifications

1. Create a workspace structure for your project:

```
my-project/
├── Cargo.toml         # Workspace manifest
├── app/               # Your application
│   └── Cargo.toml     # App manifest
└── laminar-blocks/    # Local copy of Laminar Blocks
    └── Cargo.toml     # Laminar Blocks manifest
```

2. Set up your workspace `Cargo.toml`:

```toml
[workspace]
members = [
    "app",
    "laminar-blocks"
]
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
laminar-blocks = { path = "../laminar-blocks" }
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
                    log::info!("Button clicked!");
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
2. Make sure you're generating the Tailwind CSS output file and including it in your HTML.
3. Check for any path errors in the `content` array of your Tailwind configuration.

If components aren't rendering correctly:

1. Verify you've imported the components correctly.
2. Ensure you're using the latest compatible versions of Dioxus and Laminar Blocks.
