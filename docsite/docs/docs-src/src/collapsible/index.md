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
