use crate::{use_id_or, use_unique_id};
use dioxus::prelude::*;
use lucide_dioxus::LoaderCircle;

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
    variant: ButtonVariant,

    /// The size of the button
    #[props(default)]
    size: ButtonSize,

    /// Whether the button is disabled
    #[props(default)]
    disabled: bool,

    /// Whether the button is in a loading state
    #[props(default)]
    loading: bool,

    /// Whether the button is displayed as a full width block
    #[props(default)]
    full_width: bool,

    /// Whether the button is an icon-only button (square with centered icon)
    /// Note: When using icon-only buttons, providing an aria-label is strongly recommended
    /// for accessibility purposes as there is no visible text to identify the button.
    #[props(default)]
    is_icon_button: bool,

    /// Callback when the button is clicked
    #[props(default)]
    on_click: Option<Callback<MouseEvent>>,

    /// Name of the button for form submission
    #[props(default)]
    name: String,

    /// Value of the button for form submission
    #[props(default)]
    value: String,

    /// Optional ID for the button
    #[props(default)]
    id: Option<String>,

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
    let props_id_signal = use_signal(|| props.id);
    let id_value = use_id_or(button_id, props_id_signal.into());

    // Check if icon button has aria label for accessibility
    #[cfg(debug_assertions)]
    {
        if props.is_icon_button && props.aria_label.is_none() && props.aria_labelledby.is_none() {
            log::warn!(
                "Icon button missing aria-label or aria-labelledby attribute. This may cause accessibility issues."
            );
        }
    }

    // Determine base classes for button based on variant
    let variant_classes = match props.variant {
        ButtonVariant::Primary => {
            "bg-primary text-primary-foreground hover:bg-primary/90 border-transparent focus:ring-ring"
        }
        ButtonVariant::Secondary => {
            "bg-secondary text-secondary-foreground hover:bg-secondary/80 border-transparent focus:ring-ring"
        }
        ButtonVariant::Outline => {
            "bg-background text-foreground hover:bg-muted border-border focus:ring-ring"
        }
        ButtonVariant::Ghost => {
            "bg-transparent text-foreground hover:bg-muted border-transparent focus:ring-ring"
        }
        ButtonVariant::Link => {
            "bg-transparent text-primary underline-offset-4 underline border-transparent p-0 shadow-none focus:ring-ring"
        }
        ButtonVariant::Destructive => {
            "bg-destructive text-primary-foreground dark:text-foreground hover:bg-destructive/90 border-transparent focus:ring-ring"
        }
    };

    // Determine size classes based on whether it's an icon button or regular button
    let size_classes = if props.is_icon_button {
        match props.size {
            ButtonSize::Small => "p-1.5 text-sm",
            ButtonSize::Medium => "p-2 text-base",
            ButtonSize::Large => "p-3 text-lg",
        }
    } else {
        match props.size {
            ButtonSize::Small => "text-xs px-2.5 py-1",
            ButtonSize::Medium => "text-sm px-4 py-1.5",
            ButtonSize::Large => "text-base px-6 py-2",
        }
    };

    // Determine if the button should be full width (only for non-icon buttons)
    let width_class = if props.is_icon_button {
        "w-auto" // Icon buttons should never be full width
    } else if props.full_width {
        "w-full"
    } else {
        "w-auto"
    };

    // Determine disabled and loading state classes
    let state_class = if props.disabled || props.loading {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };

    // Generate all the classes in a more maintainable way
    let button_classes = vec![
        // Base classes that apply to all buttons
        "inline-flex items-center justify-center font-medium rounded border",
        "transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
        // Variant-specific classes
        variant_classes,
        // Size-specific classes
        size_classes,
        // Width class
        width_class,
        // Icon button gets aspect-square class
        if props.is_icon_button {
            "aspect-square"
        } else {
            ""
        },
        // State class (disabled/loading)
        state_class,
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

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
            name: props.name,
            value: props.value,
            disabled: props.disabled || props.loading,
            class: button_classes,
            onclick: handle_click,

            // ARIA attributes
            aria_label: if props.is_icon_button && props.aria_label.is_none() {
                // Fallback for icon buttons without aria-label
                Some("Button".to_string())
            } else {
                props.aria_label.clone()
            },
            aria_labelledby: props.aria_labelledby.clone(),
            aria_describedby: props.aria_describedby.clone(),
            aria_controls: props.aria_controls.clone(),
            aria_expanded: props.aria_expanded.map(|v| v.to_string()),
            aria_pressed: props.aria_pressed.map(|v| v.to_string()),
            aria_disabled: (props.disabled || props.loading).to_string(),

            // Pass through other attributes
            ..props.attributes,

            if props.is_icon_button {
                // Icon button content
                if props.loading {
                    // Loading spinner for icon button
                    span {
                        class: "animate-spin inline-block",
                        aria_hidden: "true",
                        LoaderCircle {
                            class: "h-4 w-4",
                        }
                    }
                } else {
                    // Icon only when not loading
                    {props.children}
                }
            } else {
                // Standard button content
                if props.loading {
                    // Loading spinner for standard button
                    span {
                        LoaderCircle {
                            class: "mr-1 inline-block animate-spin h-4",
                        }
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

                // Button content (always shown for standard buttons)
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
}
