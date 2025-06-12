# Button

Buttons allow users to trigger actions with a single click or tap. They communicate calls to action and allow users to interact with pages in different ways.

## Button Variants

Buttons come in several variants to express different levels of emphasis in your interface.

```inject-dioxus
DemoFrame {
    button_examples::variants::ButtonVariantsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/button_examples.rs:variants}}
```

- **Primary Button**: Use primary buttons for the principal call to action on a page. Limit primary buttons to one per page to avoid competing calls to action.
- **Secondary Button**: Secondary buttons provide additional actions that are important but not the primary focus.
- **Outline Button**: Outline buttons are used for actions that need attention but aren't the primary action.
- **Ghost Button**: Ghost buttons are useful for less prominent actions, especially in busy interfaces where too many prominent buttons would be distracting.
- **Link Button**: Link buttons appear as links but maintain the same size and padding as regular buttons for consistent alignment.
- **Destructive Button**: Destructive buttons indicate actions that may have destructive consequences, like deleting data.

## Button Sizes

Buttons come in three sizes to accommodate different contexts.

```inject-dioxus
DemoFrame {
    button_examples::sizes::ButtonSizesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/button_examples.rs:sizes}}
```

## Button States

Buttons can have different states to provide feedback to users.

```inject-dioxus
DemoFrame {
    button_examples::states::ButtonStatesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/button_examples.rs:states}}
```

- **Disabled Buttons**: Show that an action exists, but cannot be performed in the current context.
- **Loading Buttons**: Provide feedback when an action takes time to complete, preventing multiple submissions.

## Buttons with Icons

Icons can enhance buttons by providing visual cues about their actions.

```inject-dioxus
DemoFrame {
    button_examples::icons::ButtonWithIconsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/button_examples.rs:icons}}
```

## Full Width Button

Full width buttons extend to the width of their container, useful for mobile interfaces or when you want to emphasize an action.

```inject-dioxus
DemoFrame {
    button_examples::full_width::FullWidthButtonExample {}
}
```

```rust, no_run
{{#include src/doc_examples/button_examples.rs:full_width}}
```

## Icon Buttons

Icon-only buttons are compact and ideal for toolbars or when space is limited. Always include an `aria_label` for accessibility.

```inject-dioxus
DemoFrame {
    button_examples::icon_buttons::IconButtonsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/button_examples.rs:icon_buttons}}
```
