# Hover Card

Hover cards display floating content when users hover over a trigger element. They're useful for providing additional context, previews, or interactive elements without requiring a click.

## Basic Usage

Hover cards appear when a user hovers over a trigger element and disappear when the pointer moves away.

```inject-dioxus
DemoFrame {
    hover_card_examples::basic::HoverCardBasicExample {}
}
```

```rust, no_run
{{#include src/doc_examples/hover_card_examples.rs:basic}}
```

## User Profile Card

A common use case for hover cards is to display user profile information when hovering over a username or avatar.

```inject-dioxus
DemoFrame {
    hover_card_examples::profile::HoverCardProfileExample {}
}
```

```rust, no_run
{{#include src/doc_examples/hover_card_examples.rs:profile}}
```

## Simple Link Tooltip

Hover cards can be used to provide additional context for links, similar to tooltips but with richer content.

```inject-dioxus
DemoFrame {
    hover_card_examples::tooltip::HoverCardTooltipExample {}
}
```

```rust, no_run
{{#include src/doc_examples/hover_card_examples.rs:tooltip}}
```

## Icon Information Card

Use hover cards to provide explanatory information when users hover over icons or UI elements that might not be self-explanatory.

```inject-dioxus
DemoFrame {
    hover_card_examples::icon::HoverCardIconExample {}
}
```

```rust, no_run
{{#include src/doc_examples/hover_card_examples.rs:icon}}
```
