# Menubar

The Menubar component provides a horizontal navigation system typically found in desktop applications. It organizes commands into logical groups, making it easy for users to find and execute actions within your application.

## Basic Usage

A basic menubar consists of multiple menus, each with a trigger and dropdown content containing menu items.

```inject-dioxus
DemoFrame {
    menubar_examples::basic::BasicMenubarExample {}
}
```

```rust, no_run
{{#include src/doc_examples/menubar_examples.rs:basic}}
```

The menubar consists of the following components:

- **Menubar**: The root container component that houses all menus.
- **MenubarMenu**: Individual menu sections with a unique index.
- **MenubarTrigger**: The clickable element that opens the menu.
- **MenubarContent**: The dropdown content container.
- **MenubarItem**: Individual selectable menu options.

## Disabled States

Menus and menu items can be disabled to indicate that they are not currently available.

```inject-dioxus
DemoFrame {
    menubar_examples::disabled::DisabledMenubarExample {}
}
```

```rust, no_run
{{#include src/doc_examples/menubar_examples.rs:disabled}}
```

You can disable entire menus or individual menu items by adding `class: Some("opacity-50 pointer-events-none".to_string())` to the respective component. This provides visual feedback to users that certain options are unavailable.

## Using Icons

Adding icons to menu items can improve usability by providing visual cues for actions.

```inject-dioxus
DemoFrame {
    menubar_examples::with_icons::MenubarWithIconsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/menubar_examples.rs:with_icons}}
```

Icons can be added to menu items by including them as children alongside text elements. For proper alignment, add a `class` with flexbox utilities like `"flex items-center gap-2"` to the `MenubarItem`.
