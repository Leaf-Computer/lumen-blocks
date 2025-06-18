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

## Use Cases

### Settings Panel

Side sheets are perfect for settings panels that need to provide various configuration options without navigating away from the current view.

```inject-dioxus
DemoFrame {
    side_sheet_examples::settings::SettingsSideSheetExample {}
}
```

```rust, no_run
{{#include src/doc_examples/side_sheet_examples.rs:settings}}
```

### Help and Documentation

Side sheets can provide contextual help or documentation without disrupting the user's workflow.

```inject-dioxus
DemoFrame {
    side_sheet_examples::help::HelpSideSheetExample {}
}
```

```rust, no_run
{{#include src/doc_examples/side_sheet_examples.rs:help}}
```

## Accessibility

The Side Sheet component follows accessibility best practices:

- Proper WAI-ARIA attributes are applied automatically
- Focus is trapped within the side sheet when open
- The side sheet can be closed by pressing the Escape key
- Screen readers announce the side sheet appropriately

## Best Practices

- Use side sheets for secondary information or controls that complement the main content
- Avoid putting critical functionality exclusively in side sheets
- Consider mobile users – ensure the side sheet is appropriately sized for smaller screens
- Use the appropriate side (left/right) based on the content's purpose:
  - Left for primary navigation or global filters
  - Right for contextual details or actions related to the current view
- Include a clear way to close the side sheet (close button, cancel button, or clicking outside)
- Structure content with header, body, and footer sections for consistent layout