# Checkbox

Checkboxes allow users to select one or more items from a set or to toggle a single option on or off. They provide a way to present multiple options that can be selected independently.

## Basic Usage

Checkboxes can be used to represent a boolean state, allowing users to toggle between checked and unchecked states.

```inject-dioxus
DemoFrame {
    checkbox_examples::basic::BasicCheckboxExample {}
}
```

```rust, no_run
{{#include src/doc_examples/checkbox_examples.rs:basic}}
```

The checkbox component requires a `checked` signal to manage its state and an `on_checked_change` handler to update that state. It's recommended to provide an `aria_label` for accessibility when the checkbox doesn't have an associated visible label.

## Checkbox Sizes

Checkboxes come in three sizes to accommodate different contexts.

```inject-dioxus
DemoFrame {
    checkbox_examples::sizes::CheckboxSizesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/checkbox_examples.rs:sizes}}
```

- **Small**: Compact size suitable for dense interfaces or when space is limited.
- **Medium**: Default size for most general use cases.
- **Large**: Larger size for improved visibility or touch targets.

## Checkbox States

Checkboxes can be disabled to indicate that the option exists but cannot be interacted with in the current context.

```inject-dioxus
DemoFrame {
    checkbox_examples::states::CheckboxStatesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/checkbox_examples.rs:states}}
```

Disabled checkboxes maintain their checked/unchecked state but prevent user interaction, visually indicating that they are not currently actionable.

## Form Integration

Checkboxes integrate seamlessly with forms by providing `id` and `name` attributes.

```inject-dioxus
DemoFrame {
    checkbox_examples::form_integration::FormIntegrationExample {}
}
```

```rust, no_run
{{#include src/doc_examples/checkbox_examples.rs:form_integration}}
```

When used in forms, checkboxes should:
- Have an associated label connected via the `for` attribute
- Include a `name` attribute to identify the value in form submissions
- Use the checked state to enable/disable submit buttons when representing required agreements

This pattern is especially useful for terms acceptance, newsletter subscriptions, and other opt-in features in your application.
