# Form Components

Form components help users input, select, and submit data. They provide structure, validation, and feedback to create intuitive user experiences.

## Input Component

The Input component allows users to enter data in forms. It supports various types like text, email, password, and more.

```inject-dioxus
DemoFrame {
    form_examples::basic::BasicInputExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:basic}}
```

## Input Variants

Inputs come in different variants to convey different states and purposes.

```inject-dioxus
DemoFrame {
    form_examples::variants::InputVariantsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:variants}}
```

- **Default Input**: Standard input for most form fields.
- **Error Input**: Indicates validation errors or problems with the input value.

## Input Sizes

Inputs are available in three sizes to fit different layout requirements.

```inject-dioxus
DemoFrame {
    form_examples::sizes::InputSizesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:sizes}}
```

## Input with Icons

Icons can enhance inputs by providing visual cues about the expected content.

```inject-dioxus
DemoFrame {
    form_examples::icons::InputWithIconsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:icons}}
```

## Input States

Inputs can have different states to provide feedback and functionality to users.

```inject-dioxus
DemoFrame {
    form_examples::states::InputStatesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:states}}
```

- **Disabled Inputs**: Show that a field exists but cannot be interacted with in the current context.
- **Read-only Inputs**: Display information that cannot be modified but should be visible to the user.

## Label Component

Labels provide context and identification for form controls, improving accessibility and usability.

```inject-dioxus
DemoFrame {
    form_examples::labels::LabelExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:labels}}
```

## Label Sizes

Labels come in three sizes to match their associated input elements.

```inject-dioxus
DemoFrame {
    form_examples::label_sizes::LabelSizesExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:label_sizes}}
```

## Required Fields

Labels can indicate when fields are required, helping users understand form requirements.

```inject-dioxus
DemoFrame {
    form_examples::required::RequiredFieldExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:required}}
```

## Form Validation

Form validation helps users provide correct information and prevents submission of invalid data.

```inject-dioxus
DemoFrame {
    form_examples::validation::FormValidationExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:validation}}
```

## Complete Form Example

This example demonstrates a complete form with validation, different input types, and submission handling.

```inject-dioxus
DemoFrame {
    form_examples::complete::CompleteFormExample {}
}
```

```rust, no_run
{{#include src/doc_examples/form_examples.rs:complete}}
```
