# Aspect Ratio

The AspectRatio component allows you to constrain content to a specific aspect ratio, ensuring consistent proportions across different screen sizes and devices. This is particularly useful for images, videos, maps, and other media that require specific dimensional relationships.

## Basic Usage

The AspectRatio component maintains a consistent width-to-height ratio regardless of the container size.

```inject-dioxus
DemoFrame {
    aspect_ratio_examples::basic_examples::BasicAspectRatioExample {}
}
```

```rust, no_run
{{#include src/doc_examples/aspect_ratio_examples.rs:basic}}
```

The `ratio` prop accepts a floating-point number representing the width-to-height ratio. For example, `16.0 / 9.0` creates a 16:9 aspect ratio.

## Common Aspect Ratios

### 4:3 Classic Photo

The 4:3 ratio is a traditional photo format, commonly used in older displays and standard photography.

```inject-dioxus
DemoFrame {
    aspect_ratio_examples::photo_example::ClassicPhotoExample {}
}
```

```rust, no_run
{{#include src/doc_examples/aspect_ratio_examples.rs:photo}}
```

### 1:1 Square Format

A perfect square format (1:1) is ideal for profile pictures, social media posts, and any content that requires equal dimensions.

```inject-dioxus
DemoFrame {
    aspect_ratio_examples::square_example::SquareFormatExample {}
}
```

```rust, no_run
{{#include src/doc_examples/aspect_ratio_examples.rs:square}}
```

### 21:9 Ultrawide Format

The 21:9 ratio provides a cinematic ultrawide format, perfect for landscape photography and video content.

```inject-dioxus
DemoFrame {
    aspect_ratio_examples::ultrawide_example::UltrawideFormatExample {}
}
```

```rust, no_run
{{#include src/doc_examples/aspect_ratio_examples.rs:ultrawide}}
```

## Custom Content

The AspectRatio component can contain any content, not just images. This makes it versatile for creating uniformly sized cards, containers, or interactive elements.

```inject-dioxus
DemoFrame {
    aspect_ratio_examples::custom_content::CustomContentExample {}
}
```

```rust, no_run
{{#include src/doc_examples/aspect_ratio_examples.rs:custom}}
```

## Multiple Aspect Ratios

You can use multiple AspectRatio components together in a grid or layout to create visually balanced designs.

```inject-dioxus
DemoFrame {
    aspect_ratio_examples::multiple_ratios::MultipleRatiosExample {}
}
```

```rust, no_run
{{#include src/doc_examples/aspect_ratio_examples.rs:multiple}}
```

## Best Practices

- Use AspectRatio to maintain consistent proportions across different screen sizes
- Consider common aspect ratios for specific content types:
  - 16:9 for videos and modern displays
  - 4:3 for traditional photos
  - 1:1 for profile pictures and social media content
  - 21:9 for cinematic/ultrawide content
  - 9:16 for mobile/portrait content
- Combine AspectRatio with responsive layouts for optimal viewing experiences
- Use the `class` prop to style the container while maintaining the specified aspect ratio
