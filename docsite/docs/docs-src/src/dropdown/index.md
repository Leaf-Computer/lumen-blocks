# Dropdown

Dropdowns display a list of actions or options that users can choose from. They provide a space-efficient way to present multiple choices in a compact UI component.

## Basic Dropdown

A basic dropdown consists of a trigger and content. The trigger can be any element, often a button, that shows and hides the dropdown content.

```inject-dioxus
DemoFrame {
    dropdown_examples::basic::BasicDropdownExample {}
}
```

```rust, no_run
{{#include src/doc_examples/dropdown_examples.rs:basic}}
```

- **Simple Dropdown**: Contains a list of items that users can select.
- **Dropdown with Labels and Separators**: Organize items into sections with labels and separate them with dividers.

## Dropdown States

Dropdowns and their items can have different states to provide feedback or limit user interaction.

```inject-dioxus
DemoFrame {
    dropdown_examples::states::DropdownStatesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/dropdown_examples.rs:states}}
```

- **Disabled Dropdown**: The entire dropdown is non-interactive.
- **Disabled Items**: Specific items within the dropdown can be disabled while others remain interactive.
- **Destructive Items**: Items that perform potentially harmful actions are styled differently to warn users.

## Dropdowns with Icons

Icons can enhance the visual appearance of dropdown items and provide additional context.

```inject-dioxus
DemoFrame {
    dropdown_examples::icons::DropdownWithIconsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/dropdown_examples.rs:icons}}
```

Icons make dropdown items more recognizable and can help users quickly identify actions.

## Dropdown Alignment

Control how the dropdown content aligns with its trigger for better layout control.

```inject-dioxus
DemoFrame {
    dropdown_examples::alignment::DropdownAlignmentExample {}
}
```

```rust, no_run
{{#include src/doc_examples/dropdown_examples.rs:alignment}}
```

- **Left Aligned (start)**: Aligns the dropdown content with the left edge of the trigger (default).
- **Center Aligned**: Centers the dropdown content relative to the trigger.
- **Right Aligned (end)**: Aligns the dropdown content with the right edge of the trigger.

## Checkbox and Radio Options

Dropdowns can contain more complex controls like checkboxes and radio buttons for selecting multiple options or choosing from a set of mutually exclusive options.

```inject-dioxus
DemoFrame {
    dropdown_examples::checkbox_radio::DropdownCheckboxRadioExample {}
}
```

```rust, no_run
{{#include src/doc_examples/dropdown_examples.rs:checkbox_radio}}
```

- **Checkbox Items**: Allow users to toggle multiple options independently.
- **Radio Items**: Let users select one option from a group of mutually exclusive choices.

## Complex Dropdown Example

Combine various dropdown elements to create rich, structured menus for complex interfaces.

```inject-dioxus
DemoFrame {
    dropdown_examples::complex::ComplexDropdownExample {}
}
```

```rust, no_run
{{#include src/doc_examples/dropdown_examples.rs:complex}}
```

This example demonstrates how to build a comprehensive dropdown menu with sections, icons, and a destructive action.