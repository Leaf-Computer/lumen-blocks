use crate::{use_id_or, use_unique_id};
use dioxus_lib::prelude::*;

/// Button variant types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Link,
    Destructive,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

/// Button size options
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// The button type (submit, reset, button)
    #[props(default = String::from("button"))]
    button_type: String,

    /// The variant of the button
    #[props(default)]
    variant: ReadOnlySignal<ButtonVariant>,

    /// The size of the button
    #[props(default)]
    size: ReadOnlySignal<ButtonSize>,
    
    /// Whether the button is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Whether the button is in a loading state
    #[props(default)]
    loading: ReadOnlySignal<bool>,

    /// Whether the button is displayed as a full width block
    #[props(default)]
    full_width: ReadOnlySignal<bool>,

    /// Callback when the button is clicked
    #[props(default)]
    on_click: Option<Callback<MouseEvent>>,

    /// Name of the button for form submission
    #[props(default)]
    name: ReadOnlySignal<String>,
    
    /// Value of the button for form submission
    #[props(default)]
    value: ReadOnlySignal<String>,

    /// Optional ID for the button
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    /// Optional icon to display before the button text
    #[props(default)]
    icon_left: Option<Element>,

    /// Optional icon to display after the button text
    #[props(default)]
    icon_right: Option<Element>,

    /// Optional aria-label for the button (for accessibility)
    #[props(default)]
    aria_label: Option<String>,

    /// Optional ID of the element that labels this button (for accessibility)
    #[props(default)]
    aria_labelledby: Option<String>,

    /// Optional ID of the element that describes this button (for accessibility)
    #[props(default)]
    aria_describedby: Option<String>,

    /// Optional aria-controls attribute
    #[props(default)]
    aria_controls: Option<String>,

    /// Optional aria-expanded attribute
    #[props(default)]
    aria_expanded: Option<bool>,

    /// Optional aria-pressed attribute
    #[props(default)]
    aria_pressed: Option<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    // Generate unique ID if not provided
    let button_id = use_unique_id();
    let id_value = use_id_or(button_id, props.id);

    // Determine base classes for button based on variant
    let variant_classes = match (props.variant)() {
        ButtonVariant::Primary => "bg-primary text-primary-foreground hover:bg-primary/90 border-transparent focus:ring-ring",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80 border-transparent focus:ring-ring",
        ButtonVariant::Outline => "bg-background text-foreground hover:bg-muted border-border focus:ring-ring",
        ButtonVariant::Ghost => "bg-transparent text-foreground hover:bg-muted border-transparent focus:ring-ring",
        ButtonVariant::Link => "bg-transparent text-primary underline-offset-4 hover:underline border-transparent p-0 shadow-none focus:ring-ring",
        ButtonVariant::Destructive => "bg-destructive text-primary-foreground hover:bg-destructive/90 border-transparent focus:ring-ring",
    };

    // Determine size classes
    let size_classes = match (props.size)() {
        ButtonSize::Small => "text-xs px-2.5 py-1.5",
        ButtonSize::Medium => "text-sm px-4 py-2",
        ButtonSize::Large => "text-base px-6 py-3",
    };

    // Determine width classes
    let width_classes = if (props.full_width)() {
        "w-full"
    } else {
        "w-auto"
    };

    // Determine disabled and loading state classes
    let state_classes = if (props.disabled)() || (props.loading)() {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };

    // Build the complete class string
    let button_classes = format!(
        "inline-flex items-center justify-center font-medium rounded border transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 {}",
        format!("{} {} {} {}", variant_classes, size_classes, width_classes, state_classes)
    );

    // Handle click event
    let handle_click = move |event: MouseEvent| {
        if let Some(callback) = &props.on_click {
            callback.call(event);
        }
    };

    rsx! {
        button {
            // Standard HTML attributes
            id: id_value,
            type: props.button_type.clone(),
            name: (props.name)(),
            value: (props.value)(),
            disabled: (props.disabled)() || (props.loading)(),
            class: button_classes,
            onclick: handle_click,

            // ARIA attributes
            aria_label: props.aria_label.clone(),
            aria_labelledby: props.aria_labelledby.clone(),
            aria_describedby: props.aria_describedby.clone(),
            aria_controls: props.aria_controls.clone(),
            aria_expanded: props.aria_expanded.map(|v| v.to_string()),
            aria_pressed: props.aria_pressed.map(|v| v.to_string()),
            aria_disabled: ((props.disabled)() || (props.loading)()).to_string(),

            // Pass through other attributes
            ..props.attributes,

            // Show loading spinner if in loading state
            if (props.loading)() {
                span {
                    class: "mr-2 inline-block animate-spin",
                    aria_hidden: "true",
                    "⟳" // Simple loading indicator (could be replaced with an SVG)
                }
            }

            // Left icon if provided
            if let Some(icon) = &props.icon_left {
                span {
                    class: "mr-2",
                    aria_hidden: "true",
                    {icon.clone()}
                }
            }

            // Button content
            {props.children}

            // Right icon if provided
            if let Some(icon) = &props.icon_right {
                span {
                    class: "ml-2",
                    aria_hidden: "true",
                    {icon.clone()}
                }
            }
        }
    }
}

// IconButton component for buttons that only contain an icon
#[derive(Props, Clone, PartialEq)]
pub struct IconButtonProps {
    /// The button type (submit, reset, button)
    #[props(default = String::from("button"))]
    button_type: String,

    /// The variant of the button
    #[props(default)]
    variant: ReadOnlySignal<ButtonVariant>,

    /// The size of the button
    #[props(default)]
    size: ReadOnlySignal<ButtonSize>,
    
    /// Whether the button is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Whether the button is in a loading state
    #[props(default)]
    loading: ReadOnlySignal<bool>,

    /// Callback when the button is clicked
    #[props(default)]
    on_click: Option<Callback<MouseEvent>>,

    /// Name of the button for form submission
    #[props(default)]
    name: ReadOnlySignal<String>,

    /// Value of the button for form submission
    #[props(default)]
    value: ReadOnlySignal<String>,

    /// Optional ID for the button
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    /// Required aria-label for the button (for accessibility)
    aria_label: String,

    /// Optional ID of the element that labels this button (for accessibility)
    #[props(default)]
    aria_labelledby: Option<String>,

    /// Optional ID of the element that describes this button (for accessibility)
    #[props(default)]
    aria_describedby: Option<String>,

    /// Optional aria-controls attribute
    #[props(default)]
    aria_controls: Option<String>,

    /// Optional aria-expanded attribute
    #[props(default)]
    aria_expanded: Option<bool>,

    /// Optional aria-pressed attribute
    #[props(default)]
    aria_pressed: Option<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
    // Generate unique ID if not provided
    let button_id = use_unique_id();
    let id_value = use_id_or(button_id, props.id);

    // Determine base classes for button based on variant
    let variant_classes = match (props.variant)() {
        ButtonVariant::Primary => "bg-primary text-primary-foreground hover:bg-primary/90 border-transparent focus:ring-ring",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80 border-transparent focus:ring-ring",
        ButtonVariant::Outline => "bg-background text-foreground hover:bg-muted border-border focus:ring-ring",
        ButtonVariant::Ghost => "bg-transparent text-foreground hover:bg-muted border-transparent focus:ring-ring",
        ButtonVariant::Link => "bg-transparent text-primary underline-offset-4 hover:underline border-transparent p-0 shadow-none focus:ring-ring",
        ButtonVariant::Destructive => "bg-destructive text-primary-foreground hover:bg-destructive/90 border-transparent focus:ring-ring",
    };

    // Determine size classes for icon button (square aspect ratio)
    let size_classes = match (props.size)() {
        ButtonSize::Small => "p-1.5 text-sm",
        ButtonSize::Medium => "p-2 text-base",
        ButtonSize::Large => "p-3 text-lg",
    };

    // Determine disabled and loading state classes
    let state_classes = if (props.disabled)() || (props.loading)() {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };

    // Build the complete class string
    let button_classes = format!(
        "inline-flex items-center justify-center aspect-square font-medium rounded border transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 {}",
        format!("{} {} {}", variant_classes, size_classes, state_classes)
    );

    // Handle click event
    let handle_click = move |event: MouseEvent| {
        if let Some(callback) = &props.on_click {
            callback.call(event);
        }
    };

    rsx! {
        button {
            // Standard HTML attributes
            id: id_value,
            type: props.button_type.clone(),
            name: (props.name)(),
            value: (props.value)(),
            disabled: (props.disabled)() || (props.loading)(),
            class: button_classes,
            onclick: handle_click,

            // ARIA attributes
            aria_label: props.aria_label.clone(),
            aria_labelledby: props.aria_labelledby.clone(),
            aria_describedby: props.aria_describedby.clone(),
            aria_controls: props.aria_controls.clone(),
            aria_expanded: props.aria_expanded.map(|v| v.to_string()),
            aria_pressed: props.aria_pressed.map(|v| v.to_string()),
            aria_disabled: ((props.disabled)() || (props.loading)()).to_string(),

            // Pass through other attributes
            ..props.attributes,

            // Show loading spinner if in loading state
            if (props.loading)() {
                span {
                    class: "animate-spin",
                    aria_hidden: "true",
                    "⟳" // Simple loading indicator (could be replaced with an SVG)
                }
            } else {
                // Button content (icon)
                {props.children}
            }
        }
    }
}