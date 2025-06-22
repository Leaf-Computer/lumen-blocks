# Side Sheet

Side sheets are slide-in panels that appear from the edge of the screen, providing additional context or controls without forcing users to navigate away from the current page.

## Basic Side Sheet

A basic side sheet includes a trigger, content container, header, body, and footer sections.

```inject-dioxus
DemoFrame {
    side_sheet_examples::basic::BasicSideSheetExample {}
}
```

```rust, no_run
{{#include src/doc_examples/side_sheet_examples.rs:basic}}
```

The Side Sheet component is composed of several parts:

- **SideSheet**: The root container that manages the state of the side sheet.
- **SideSheetTrigger**: The element that toggles the side sheet open/closed.
- **SideSheetContent**: The container for all content within the side sheet.
- **SideSheetCloseButton**: An optional close button (typically an X) that appears in the top corner.
- **SideSheetHeader**: Container for the title and description.
- **SideSheetTitle**: The title of the side sheet.
- **SideSheetDescription**: Optional description text.
- **SideSheetBody**: The main content area of the side sheet.
- **SideSheetFooter**: Container for action buttons, typically at the bottom.
- **SideSheetClose**: A wrapper that automatically closes the side sheet when its children are clicked.

## Side Sheet Positions

Side sheets can slide in from either the left or right side of the screen.

```inject-dioxus
DemoFrame {
    side_sheet_examples::positions::SideSheetPositionsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/side_sheet_examples.rs:positions}}
```

- **Right Side Sheet (default)**: Commonly used for detail panels, forms, and contextual information related to the main content.
- **Left Side Sheet**: Often used for navigation menus, filters, or other controls that affect the entire page.
