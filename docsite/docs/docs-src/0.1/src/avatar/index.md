# Avatar

Avatars represent users or entities with images, initials, or icons. They help personalize interfaces and provide visual identification for user profiles, comments, or collaborative spaces.

## Basic Avatar

The basic avatar component displays a user's image with a fallback when the image fails to load.

```inject-dioxus
DemoFrame {
    avatar_examples::basic::BasicAvatarExample {}
}
```

```rust, no_run
{{#include src/doc_examples/avatar_examples.rs:basic}}
```

The Avatar component consists of:
- **AvatarImage**: Displays the user's image
- **AvatarFallback**: Shows when the image fails to load or isn't available

## Size Variations

Avatars come in different sizes to fit various UI contexts - from small indicators to prominent profile displays.

```inject-dioxus
DemoFrame {
    avatar_examples::sizes::SizeVariationsExample {}
}
```

```rust, no_run
{{#include src/doc_examples/avatar_examples.rs:sizes}}
```

Avatars can be customized with the `class` prop to define specific dimensions:
- **Small**: Ideal for compact interfaces or lists (`h-8 w-8`)
- **Medium**: Default size for most contexts
- **Large**: For prominent user representations (`h-16 w-16`)
- **Extra Large**: For profile pages or detailed views (`h-20 w-20`)

## Error State & Fallbacks

Fallbacks ensure avatars always display something meaningful when images fail to load.

```inject-dioxus
DemoFrame {
    avatar_examples::fallbacks::ErrorStateExample {}
}
```

```rust, no_run
{{#include src/doc_examples/avatar_examples.rs:fallbacks}}
```

Fallback options include:
- **Initials**: Display the user's initials (e.g., "JD" for John Doe)
- **Emoji**: Show a generic emoji like 👤
- **Icons**: Simple symbols or icons
- **Styled Fallbacks**: Colored backgrounds with contrasting text

## Avatar Groups

Avatar groups display multiple users together, useful for showing team members, participants, or collaborators.

```inject-dioxus
DemoFrame {
    avatar_examples::groups::AvatarGroupExample {}
}
```

```rust, no_run
{{#include src/doc_examples/avatar_examples.rs:groups}}
```

Two common styles for avatar groups:
- **Overlapping**: Avatars slightly overlap to save space, ideal for showing many users
- **Spaced**: Avatars with gaps between them for clearer distinction

When there are too many avatars to display, use a "+N" indicator to show the number of additional users.

## State Changes

The Avatar component provides state change notifications that can be useful for handling loading events, errors, or tracking when fallbacks are displayed.

```inject-dioxus
DemoFrame {
    avatar_examples::state::AvatarStateExample {}
}
```

```rust, no_run
{{#include src/doc_examples/avatar_examples.rs:state}}
```

The `on_state_change` prop allows you to execute code when the avatar's state changes, such as when an image loads successfully or fails to load.