# Accordion

Accordions allow users to expand and collapse sections of content, making complex information more manageable and interfaces more compact. They're ideal for FAQs, settings panels, and content that can be progressively disclosed.

## Basic Usage

The basic accordion allows only one section to be open at a time, automatically closing others when a new section is opened.

```inject-dioxus
DemoFrame {
    accordion_examples::BasicAccordionExample {}
}
```

```rust, no_run
{{#include src/doc_examples/accordion_examples.rs:basic}}
```

## Accordion Types

Accordions can be configured to behave in different ways to suit various interface requirements.

### Single Open Item

The default accordion behavior allows only one item to be open at a time, automatically closing others. This is ideal for navigation menus or when users should focus on one piece of content at a time.

```inject-dioxus
DemoFrame {
    accordion_examples::BasicAccordionExample {}
}
```

```rust, no_run
{{#include src/doc_examples/accordion_examples.rs:basic}}
```

### Multiple Open Items

When users need to compare information across sections, you can configure accordions to allow multiple items to be open simultaneously.

```inject-dioxus
DemoFrame {
    accordion_examples::MultipleOpenAccordionExample {}
}
```

```rust, no_run
{{#include src/doc_examples/accordion_examples.rs:multiple}}
```

## Accordion Structure

The accordion component is composed of several nested components that work together:

- **Accordion**: The main wrapper component that manages the state of child accordion items
- **AccordionItem**: Individual collapsible sections that contain a trigger and content
- **AccordionTrigger**: The clickable header that toggles the expansion state
- **AccordionContent**: The content revealed when an accordion item is expanded

Each `AccordionItem` requires an `index` prop that uniquely identifies it within the accordion group.

## Best Practices

- Use accordions when space is limited and information can be logically grouped into sections
- Provide clear, concise headers that accurately describe the hidden content
- Consider whether users need to compare information across sections (use multiple open items) or focus on one section at a time (single open item)
- For accessibility, ensure accordion triggers clearly indicate their expanded/collapsed state
- Don't nest accordions too deeply as this can create confusion and navigation difficulties
