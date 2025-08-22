# Installation

This guide will walk you through the process of installing and setting up Lumen Blocks (version `v0.2.0`) in your Dioxus project.

## Prerequisites

Before installing Lumen Blocks, ensure you have:

- [Rust](https://www.rust-lang.org/tools/install) installed
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/#installing-the-cli) installed

## Starting from our base project template (new projects)

Clone the starter template:
```bash
git clone https://github.com/Leaf-Computer/lumen-blocks-starter.git my-lumen-app
```

Navigate to the newly created project directory and ensure you are on a compatible revision:
```bash
cd my-lumen-app
git checkout 297c95aa09145e5a640cdae4cd4501a546963f52
```

Start the development server:
```bash
dx serve
```

Open your browser and visit `http://localhost:8080` to see your new Lumen Blocks project.


## Installing Lumen Blocks on an existing project

These options assume you have an existing Dioxus 0.7 project setup with Tailwind, similar to the one in the [Dioxus Tailwind guide](https://dioxuslabs.com/learn/0.7/cookbook/tailwind).

For an existing project, you have two options for installing Lumen Blocks:

### Option 1: Add as a Dependency

Add Lumen Blocks to your `Cargo.toml`:
```toml
# Change revision as needed
lumen-blocks = { git = "https://github.com/Leaf-Computer/lumen-blocks.git", rev = "654c76c169f8c2ee8bfc1a6406f26cd3d1c0799c" }
```

Create or update a `tailwind.css` file in your project's root directory, with the following content:
```css
@import "tailwindcss";
/* 
- We need to use the JS config format to be able to specify content paths with environment variables.
- The file is named `tailwind-config.js` and not `tailwind.config.js` on purpose, because otherwise Dioxus CLI does not know whether to use Tailwind v3 or v4 and doesn't automatically generate the output `assets/tailwind.css` file for us.
*/
@config "./tailwind-config.js";

body {
    background-color: var(--background);
    color: var(--foreground);
}

/*
Based on MIT-licensed code from shadcn: https://github.com/shadcn-ui/ui
*/

:root {
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
    --destructive: oklch(0.5757 0.2352 27.92);
    --destructive-foreground: oklch(0.577 0.245 27.325);
    --border: oklch(0.922 0 0);
    --input: oklch(0.922 0 0);
    --ring: oklch(0.708 0 0);
    --chart-1: oklch(0.646 0.222 41.116);
    --chart-2: oklch(0.6 0.118 184.704);
    --chart-3: oklch(0.398 0.07 227.392);
    --chart-4: oklch(0.828 0.189 84.429);
    --chart-5: oklch(0.769 0.188 70.08);
    --radius: 0.625rem;
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
        --card: oklch(0.145 0 0);
        --card-foreground: oklch(0.985 0 0);
        --popover: oklch(0.145 0 0);
        --popover-foreground: oklch(0.985 0 0);
        --primary: oklch(0.985 0 0);
        --primary-foreground: oklch(0.205 0 0);
        --secondary: oklch(0.269 0 0);
        --secondary-foreground: oklch(0.985 0 0);
        --muted: oklch(0.269 0 0);
        --muted-foreground: oklch(0.708 0 0);
        --accent: oklch(0.269 0 0);
        --accent-foreground: oklch(0.985 0 0);
        --destructive: oklch(0.5058 0.2066 27.85);
        --border: oklch(0.269 0 0);
        --input: oklch(0.269 0 0);
        --ring: oklch(0.439 0 0);
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
        --sidebar-border: oklch(0.269 0 0);
        --sidebar-ring: oklch(0.439 0 0);
    }
}

@theme inline {
    /* Colors */
    --color-background: var(--background);
    --color-foreground: var(--foreground);
    --color-card: var(--card);
    --color-card-foreground: var(--card-foreground);
    --color-popover: var(--popover);
    --color-popover-foreground: var(--popover-foreground);
    --color-primary: var(--primary);
    --color-primary-foreground: var(--primary-foreground);
    --color-secondary: var(--secondary);
    --color-secondary-foreground: var(--secondary-foreground);
    --color-muted: var(--muted);
    --color-muted-foreground: var(--muted-foreground);
    --color-accent: var(--accent);
    --color-accent-foreground: var(--accent-foreground);
    --color-destructive: var(--destructive);
    --color-destructive-foreground: var(--primary-foreground);
    --color-border: var(--border);
    --color-input: var(--input);
    --color-ring: var(--ring);
    --color-chart-1: var(--chart-1);
    --color-chart-2: var(--chart-2);
    --color-chart-3: var(--chart-3);
    --color-chart-4: var(--chart-4);
    --color-chart-5: var(--chart-5);
    --radius-sm: calc(var(--radius) - 4px);
    --radius-md: calc(var(--radius) - 2px);
    --radius-lg: var(--radius);
    --radius-xl: calc(var(--radius) + 4px);
    --color-sidebar: var(--sidebar);
    --color-sidebar-foreground: var(--sidebar-foreground);
    --color-sidebar-primary: var(--sidebar-primary);
    --color-sidebar-primary-foreground: var(--sidebar-primary-foreground);
    --color-sidebar-accent: var(--sidebar-accent);
    --color-sidebar-accent-foreground: var(--sidebar-accent-foreground);
    --color-sidebar-border: var(--sidebar-border);
    --color-sidebar-ring: var(--sidebar-ring);
    
    /* Animation definitions */
    --animate-slide-in-from-right: slide-in-from-right 0.2s ease-out;
    --animate-slide-out-to-right: slide-out-to-right 0.2s ease-out;

    /* Animation behavior utilities */
    --animate-in: forwards cubic-bezier(0.16, 1, 0.3, 1);
    --animate-out: forwards cubic-bezier(0.16, 1, 0.3, 1);

    /* Keyframes definitions */
    @keyframes slide-in-from-right {
        from { transform: translateX(100%); }
        to { transform: translateX(0); }
    }
    
    @keyframes slide-out-to-right {
        from { transform: translateX(0); }
        to { transform: translateX(100%); }
    }
}
```

Create a `tailwind-config.js` to include all search paths:
```js
module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
    // Include Lumen Blocks components
    // Note: The `654c76c` on the path is there to match the Lumen Blocks version in `Cargo.toml`. If you update Lumen Blocks on your project, you should update this path as well with the first 7 digits of the commit hash.
    `${process.env.HOME}/.cargo/git/checkouts/lumen-blocks-*/654c76c/blocks/src/**/*.rs`
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
```

**Note:** We intentionally use the legacy JS configuration format to be able to use environment variables on the content paths, and we also intentionally use the name `tailwind-config.js` and not the standard `tailwind.config.js` because otherwise the Dioxus CLI does not know whether to use Tailwind v3 or v4 and fails to generate the output Tailwind configuration.

### Option 2: Other configurations

You can also choose to import the project files directly on your project tree manually, via git submodules, or some other mechanism. This may be useful in scenarios where you would like to tweak the base components themselves.

This is more advanced but many of the steps in the previous section apply here. Here are the general steps you must follow in almost every possible setup:
1. Import the blocks into your `Cargo.toml`
2. Create a `tailwind.css` similar to the one in the previous section.
3. Create a `tailwind-config.js` file similar to the one in the previous section, but modify the path to the blocks based on where you installed the blocks. Ensure it is named `tailwind-config.js` and not `tailwind.config.js` to avoid confusing the Dioxus CLI when it tries to do its automatic Tailwind generation.

## Using Lumen Blocks

After installation, you can import and use Lumen Blocks components in your Dioxus application. Please see the other documentation pages for examples!

## Troubleshooting

If Tailwind CSS classes aren't being applied:

- Ensure your `tailwind-config.js` correctly points to both your source files and the Lumen Blocks components.
- Make sure Dioxus CLI is automatically generating the Tailwind CSS output file.
- Make sure you are importing your **output** `tailwind.css` in your application.
- Check for any path errors in the `content` array of your Tailwind configuration.

If components aren't rendering correctly:

- Verify you've imported the components correctly.
- Ensure you're using the latest compatible versions of Dioxus and Lumen Blocks.

If you are still running into issues, please open an issue here: [Issues](https://github.com/Leaf-Computer/lumen-blocks/issues)
