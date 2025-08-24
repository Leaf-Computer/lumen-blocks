# Migration notes

In this document you will find notes and tips for migrating between different versions of Lumen Blocks.

## v0.1 to v0.2

### Biggest differences

Here are the major differences between Lumen Blocks `v0.1.x` and `v0.2.x`:
1. `v0.2.x` uses Dioxus 0.7 instead of Dioxus 0.6.
2. `v0.2.x` uses Tailwind 4 instead of Tailwind 3.
3. Minor changes to the interface in some components.

### Migration guide

To migrate from Lumen Blocks `v0.1.x` to Lumen Blocks `v0.2.x`:
1. Update your Dioxus CLI to the proper 0.7 version.
2. Update your dependencies to use Dioxus 0.7, and resolve any incompatibilities that arise.
3. Update your dependencies to use the new Lumen Blocks.
4. Refactor your `tailwind.css` to use Tailwind v4, using the one shown in the [Installation Guide](../installation/index.md) as a guide _(or replace it entirely if you haven't modified it much since the start)_.
5. Rename `tailwind.config.js` to `tailwind-config.js`, and refactor it using the one shown in the [Installation Guide](../installation/index.md) as a guide. You will notice the new config file is much shorter. This is because most of the configuration moved to `tailwind.css` itself.
6. Update usages of the [Dropdown](../dropdown/index.md) and [Menubar](../menubar/index.md) components using this documentation as a reference, as they have changed slightly.
