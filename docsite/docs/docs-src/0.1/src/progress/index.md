# Progress

Progress components provide visual feedback about the completion status of tasks or operations, helping users understand how far along they are in a process.

## Basic Progress

A simple progress bar displays the completion percentage of a task.

```inject-dioxus
DemoFrame {
    progress_examples::basic::BasicProgressExample {}
}
```

```rust, no_run
{{#include src/doc_examples/progress_examples.rs:basic}}
```

## Progress with Percentage Display

Progress bars can display the current percentage to provide more precise information.

```inject-dioxus
DemoFrame {
    progress_examples::percentages::ProgressWithPercentageExample {}
}
```

```rust, no_run
{{#include src/doc_examples/progress_examples.rs:percentages}}
```

## Progress Sizes

Progress bars come in three sizes to fit different contexts.

```inject-dioxus
DemoFrame {
    progress_examples::sizes::ProgressSizesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/progress_examples.rs:sizes}}
```

- **Small**: Compact progress bars for space-constrained interfaces or when visual prominence isn't required.
- **Medium**: The default size, suitable for most use cases.
- **Large**: Higher visibility progress bars for when you want to emphasize the progress status.

## Progress Variants

Different color variants help convey the context or status of the progress.

```inject-dioxus
DemoFrame {
    progress_examples::variants::ProgressVariantsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/progress_examples.rs:variants}}
```

- **Default**: Standard progress indicator for neutral operations.
- **Success**: Green progress bar for completed or successful operations.
- **Warning**: Yellow progress bar to indicate caution or potential issues.
- **Destructive**: Red progress bar for critical or problematic situations.

## Interactive Progress

Progress bars can be updated dynamically in response to user actions or application events.

```inject-dioxus
DemoFrame {
    progress_examples::interactive::InteractiveProgressExample {}
}
```

```rust, no_run
{{#include src/doc_examples/progress_examples.rs:interactive}}
```
