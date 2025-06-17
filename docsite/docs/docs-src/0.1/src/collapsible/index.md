# Collapsible

The Collapsible component provides an expandable/collapsible container that reveals or hides content. It's commonly used for FAQs, accordion menus, progressive disclosure patterns, and anywhere you need to conserve space while keeping information accessible.

## Basic Collapsible

The basic collapsible component consists of a trigger that users can click to expand or collapse the associated content.

```inject-dioxus
DemoFrame {
    collapsible_examples::basic::BasicCollapsibleExample {}
}
```

```rust, no_run
{{#include src/doc_examples/collapsible_examples.rs:basic}}
```

The collapsible component consists of three main parts:
- **Collapsible**: The container component that manages the state
- **CollapsibleTrigger**: The clickable element that toggles the expanded state
- **CollapsibleContent**: The content that will be shown or hidden

## Multiple Collapsibles

You can stack multiple collapsible components to create accordion-like interfaces. Each collapsible operates independently by default.

```inject-dioxus
DemoFrame {
    collapsible_examples::multiple::MultipleCollapsiblesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/collapsible_examples.rs:multiple}}
```

## Nested Collapsibles

Collapsibles can be nested to create more complex disclosure patterns and information hierarchies.

```inject-dioxus
DemoFrame {
    collapsible_examples::nested::NestedCollapsibleExample {}
}
```

```rust, no_run
{{#include src/doc_examples/collapsible_examples.rs:nested}}
```

When nesting collapsibles, consider using visual cues like indentation or borders to help users understand the hierarchy.

## Accessibility

The Collapsible component follows accessibility best practices:

- Uses appropriate ARIA attributes to communicate state to screen readers
- Supports keyboard navigation
- Maintains focus management when content is shown or hidden
- Provides visual cues for interactive elements

When implementing collapsibles, remember to:
- Use clear, descriptive labels for triggers
- Provide sufficient contrast for all interactive elements
- Test your implementation with keyboard navigation and screen readers
