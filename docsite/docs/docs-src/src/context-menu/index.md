# Context Menu

Context menus provide contextual actions to users when they right-click on elements. They offer quick access to common commands or options relevant to the clicked element.

## Basic Context Menu

A basic context menu appears when a user right-clicks on a trigger element and displays a list of actions.

```inject-dioxus
DemoFrame {
    context_menu_examples::basic::BasicContextMenuExample {}
}
```

```rust, no_run
{{#include src/doc_examples/context_menu_examples.rs:basic}}
```

## Context Menu with Labels

Labels help organize context menu items into logical groups, making it easier for users to find relevant options.

```inject-dioxus
DemoFrame {
    context_menu_examples::with_labels::ContextMenuWithLabelsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/context_menu_examples.rs:with_labels}}
```

## Context Menu with Checkboxes

Checkbox items in context menus allow users to toggle options on and off without closing the menu.

```inject-dioxus
DemoFrame {
    context_menu_examples::with_checkboxes::ContextMenuWithCheckboxesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/context_menu_examples.rs:with_checkboxes}}
```

## Context Menu with Radio Groups

Radio groups in context menus let users select one option from a set of mutually exclusive choices.

```inject-dioxus
DemoFrame {
    context_menu_examples::with_radio::ContextMenuWithRadioExample {}
}
```

```rust, no_run
{{#include src/doc_examples/context_menu_examples.rs:with_radio}}
```

## Disabled Context Menu

Context menus can be disabled when contextual actions are not available or permitted.

```inject-dioxus
DemoFrame {
    context_menu_examples::disabled::DisabledContextMenuExample {}
}
```

```rust, no_run
{{#include src/doc_examples/context_menu_examples.rs:disabled}}
```

## Components

Context menus are built using several components working together:

- **ContextMenu**: The main wrapper component that manages the context menu state.
- **ContextMenuTrigger**: Defines the area that responds to right-clicks to open the menu.
- **ContextMenuContent**: The container for all menu items, with configurable width.
- **ContextMenuItem**: Standard clickable menu items that can include icons.
- **ContextMenuCheckboxItem**: Menu items with toggleable checkbox state.
- **ContextMenuRadioGroup**: Container for radio items, managing selection state.
- **ContextMenuRadioItem**: Selectable items within a radio group.
- **ContextMenuLabel**: Text labels for grouping related menu items.
- **ContextMenuSeparator**: Visual dividers to separate groups of menu items.
