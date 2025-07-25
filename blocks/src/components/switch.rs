use crate::{use_id_or, use_unique_id};
use dioxus_lib::html::GlobalAttributesExtension;
use dioxus_lib::prelude::*;
use dioxus_primitives::switch::{Switch as PrimitiveSwitch, SwitchThumb};

/// Switch size options
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SwitchSize {
    Small,
    Medium,
    Large,
}

impl Default for SwitchSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Props for the Switch component
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Whether the switch is checked
    #[props(default)]
    pub checked: Signal<bool>,

    /// Callback for when the switch is toggled
    #[props(default)]
    pub on_checked_change: Option<EventHandler<bool>>,

    /// Whether the switch is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Size of the switch
    #[props(default)]
    pub size: SwitchSize,

    /// Optional ID for the switch
    #[props(default)]
    pub id: ReadOnlySignal<Option<String>>,

    /// Accessible label for the switch
    #[props(default)]
    pub aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A styled switch component that can be toggled on or off
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    // Generate unique ID if not provided
    let switch_id = use_unique_id();
    let id_value = use_id_or(switch_id, props.id);
    let inner_checked_state = use_memo(move || Some((props.checked)()));

    // Determine size-specific classes
    let (switch_classes, thumb_size_classes, thumb_translation) = match props.size {
        SwitchSize::Small => (
            "h-[1.25rem] w-[2.25rem]",
            "h-[1rem] w-[1rem]",
            "translate-x-[0rem] group-aria-checked:translate-x-[1rem]",
        ),
        SwitchSize::Large => (
            "h-[1.75rem] w-[3.5rem]",
            "h-[1.5rem] w-[1.5rem]",
            "translate-x-[0rem] group-aria-checked:translate-x-[1.75rem]",
        ),
        SwitchSize::Medium => (
            "h-[1.5rem] w-[2.75rem]",
            "h-[1.25rem] w-[1.25rem]",
            "translate-x-[0rem] group-aria-checked:translate-x-[1.25rem]",
        ),
    };

    // Build full switch classes
    let full_switch_classes = vec![
        // Base classes
        "group",
        "relative inline-flex shrink-0 cursor-pointer disabled:cursor-not-allowed disabled:opacity-50 rounded-full border-2 border-transparent",
        "transition-colors duration-300 ease-in-out focus:outline-none focus:ring-2",
        "focus:ring-ring focus:ring-offset-2 focus:ring-offset-background",
        "bg-input aria-checked:bg-primary",
        // Size classes
        switch_classes,
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    // Build thumb classes with dynamic position based on checked state
    let full_thumb_classes = move || {
        vec![
            // Base classes
            "pointer-events-none inline-block transform rounded-full bg-background shadow ring-0",
            // Improved transition for smoother animation
            "transition-transform duration-300 ease-in-out will-change-transform",
            // Size classes
            thumb_size_classes,
            // Position classes based on checked state
            thumb_translation,
        ]
        .join(" ")
    };

    // Handler for change events
    let on_change = move |checked: bool| {
        if let Some(handler) = &props.on_checked_change {
            handler.call(checked);
        }
    };

    rsx! {
        PrimitiveSwitch {
            id: id_value,
            class: full_switch_classes,
            checked: inner_checked_state,
            on_checked_change: on_change,
            disabled: (props.disabled)(),
            aria_label: props.aria_label.clone(),

            SwitchThumb {
                class: full_thumb_classes(),
                // Add ARIA attributes for better accessibility
                aria_hidden: "true".to_string(),
            }
        }
    }
}
