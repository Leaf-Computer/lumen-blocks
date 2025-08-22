# Toast

Toast notifications provide brief, temporary messages that appear overlaid on the interface. They're ideal for providing non-disruptive feedback about the outcomes of user actions or system events.

## Setting Up the Toast Provider

To use toasts in your application, you need to wrap a part of your component tree with the `ToastProvider`. This is typically done at a high level in your application.

```inject-dioxus
DemoFrame {
    toast_examples::setup::ToastProviderSetupExample {}
}
```

```rust, no_run
{{#include src/doc_examples/toast_examples.rs:setup}}
```

## Basic Toast Types

Toasts come in several types to represent different kinds of messages.

```inject-dioxus
DemoFrame {
    toast_examples::basic::BasicToastExample {}
}
```

```rust, no_run
{{#include src/doc_examples/toast_examples.rs:basic}}
```

- **Success Toasts**: Used to confirm that an action was completed successfully.
- **Error Toasts**: Inform users about errors or failed operations.
- **Warning Toasts**: Alert users about potential issues or important considerations.
- **Info Toasts**: Provide general information or updates to users.

## Toasts with Descriptions

Add more context to your toasts with optional descriptions.

```inject-dioxus
DemoFrame {
    toast_examples::descriptions::ToastWithDescriptionsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/toast_examples.rs:descriptions}}
```

Descriptions help provide additional context about the notification, making them more informative without overwhelming the user interface.

## Custom Duration Toasts

Control how long toasts appear on screen, from quick notifications to persistent messages.

```inject-dioxus
DemoFrame {
    toast_examples::durations::CustomDurationToastExample {}
}
```

```rust, no_run
{{#include src/doc_examples/toast_examples.rs:durations}}
```

- **Short Duration**: For simple confirmations that require minimal user attention.
- **Longer Duration**: For messages that users might need more time to read.
- **Permanent Toasts**: For critical messages that users must acknowledge by manually dismissing.

## Advanced Usage

Toasts can be used in more complex scenarios to provide rich feedback experiences.

```inject-dioxus
DemoFrame {
    toast_examples::advanced::AdvancedToastExample {}
}
```

```rust, no_run
{{#include src/doc_examples/toast_examples.rs:advanced}}
```

Advanced techniques include:
- Using the direct `show()` method for custom toast configurations
- Displaying multiple toasts to represent sequential operations
- Creating interactive process flows that update users on multi-step operations

## Best Practices

- **Be Concise**: Keep toast messages short and to the point.
- **Use Appropriate Types**: Match the toast type to the nature of the message.
- **Consider Duration**: Set durations based on the importance and complexity of the message.
- **Don't Overuse**: Too many toast notifications can overwhelm users.
- **Provide Actions When Needed**: For complex errors or warnings, consider adding action buttons to help users resolve issues.