use crate::{use_id_or, use_unique_id};
use dioxus::prelude::*;

/// Input size options
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum InputSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Input variant types
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum InputVariant {
    #[default]
    Default,
    Error,
}

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// The input type (text, email, password, etc.)
    #[props(default = String::from("text"))]
    input_type: String,

    /// Whether to autofocus
    #[props(default)]
    autofocus: bool,

    /// The variant of the input
    #[props(default)]
    variant: InputVariant,

    /// The size of the input
    #[props(default)]
    size: InputSize,

    /// Whether the input is disabled
    #[props(default)]
    disabled: bool,

    /// Whether the input is read-only
    #[props(default)]
    readonly: bool,

    /// Whether the input is required
    #[props(default)]
    required: bool,

    /// Placeholder text for the input
    #[props(default)]
    placeholder: String,

    /// Current value for the input (controlled component)
    #[props(default)]
    value: String,

    /// Whether the input is displayed as a full width block
    #[props(default)]
    full_width: bool,

    /// Optional icon to display before the input text
    #[props(default)]
    icon_left: Option<Element>,

    /// Optional icon to display after the input text
    #[props(default)]
    icon_right: Option<Element>,

    /// Callback when the input value changes
    #[props(default)]
    on_change: Option<Callback<FormEvent>>,

    /// Callback when the input value changes
    #[props(default)]
    on_input: Option<Callback<FormEvent>>,

    /// Callback when the input is focused
    #[props(default)]
    on_focus: Option<Callback<FocusEvent>>,

    /// Callback when the input loses focus
    #[props(default)]
    on_blur: Option<Callback<FocusEvent>>,

    /// Name of the input for form submission
    #[props(default)]
    name: String,

    /// Optional ID for the input
    #[props(default)]
    id: Option<String>,

    /// Optional aria-label for the input (for accessibility)
    #[props(default)]
    aria_label: Option<String>,

    /// Optional ID of the element that labels this input (for accessibility)
    #[props(default)]
    aria_labelledby: Option<String>,

    /// Optional ID of the element that describes this input (for accessibility)
    #[props(default)]
    aria_describedby: Option<String>,

    /// Optional additional classes for the input
    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    // Generate unique ID if not provided
    let input_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(input_id, props_id.into());

    // Determine variant classes
    let variant_classes = match props.variant {
        InputVariant::Default => "border-input focus:border-ring",
        InputVariant::Error => "border-destructive focus:border-destructive",
    };

    // Determine size classes
    let size_classes = match props.size {
        InputSize::Small => "text-xs px-2 py-1 h-8",
        InputSize::Medium => "text-sm px-3 py-1.5 h-10",
        InputSize::Large => "text-base px-4 py-2 h-12",
    };

    // Determine width class
    let width_class = if props.full_width { "w-full" } else { "w-auto" };

    // Determine state classes
    let state_class = if props.disabled {
        "opacity-50 cursor-not-allowed bg-muted"
    } else {
        "bg-background"
    };

    // Padding adjustment when icons are present
    let padding_left = if props.icon_left.is_some() {
        match props.size {
            InputSize::Small => "pl-7",
            InputSize::Medium => "pl-9",
            InputSize::Large => "pl-10",
        }
    } else {
        ""
    };

    let padding_right = if props.icon_right.is_some() {
        match props.size {
            InputSize::Small => "pr-7",
            InputSize::Medium => "pr-9",
            InputSize::Large => "pr-10",
        }
    } else {
        ""
    };

    // Generate all the classes
    let input_classes = vec![
        // Base classes
        "rounded border text-foreground",
        "transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2",
        // Variant-specific classes
        variant_classes,
        // Size-specific classes
        size_classes,
        // Width class
        width_class,
        // State class
        state_class,
        // Padding adjustments for icons
        padding_left,
        padding_right,
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    // Handle change event
    let handle_change = move |event: FormEvent| {
        if let Some(callback) = &props.on_change {
            callback.call(event);
        }
    };

    // Handle input event
    let handle_input = move |event: FormEvent| {
        if let Some(callback) = &props.on_input {
            callback.call(event);
        }
    };

    // Handle focus event
    let handle_focus = move |event: FocusEvent| {
        if let Some(callback) = &props.on_focus {
            callback.call(event);
        }
    };

    // Handle blur event
    let handle_blur = move |event: FocusEvent| {
        if let Some(callback) = &props.on_blur {
            callback.call(event);
        }
    };

    rsx! {
        div {
            class: "relative",

            // Left icon if provided
            if let Some(icon) = &props.icon_left {
                div {
                    class: "absolute left-0 inset-y-0 flex items-center pl-2 text-foreground",
                    aria_hidden: "true",
                    {icon.clone()}
                }
            }

            input {
                // Standard HTML attributes
                id: id_value,
                type: props.input_type.clone(),
                autofocus: props.autofocus,
                name: props.name,
                placeholder: props.placeholder,
                value: props.value,
                disabled: props.disabled,
                readonly: props.readonly,
                required: props.required,
                class: input_classes,

                // Event handlers
                onchange: handle_change,
                oninput: handle_input,
                onfocus: handle_focus,
                onblur: handle_blur,

                // ARIA attributes
                aria_label: props.aria_label.clone(),
                aria_labelledby: props.aria_labelledby.clone(),
                aria_describedby: props.aria_describedby.clone(),
                aria_disabled: props.disabled.to_string(),
                aria_required: props.required.to_string(),

                // Pass through other attributes
                ..props.attributes,
            }

            // Right icon if provided
            if let Some(icon) = &props.icon_right {
                div {
                    class: "absolute right-0 inset-y-0 flex items-center pr-2 text-foreground",
                    aria_hidden: "true",
                    {icon.clone()}
                }
            }
        }
    }
}
