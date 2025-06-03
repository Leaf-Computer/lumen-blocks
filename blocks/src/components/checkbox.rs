use crate::use_unique_id;
use dioxus_lib::prelude::*;
use lucide_dioxus::Check;

/// Checkbox size options
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CheckboxSize {
    Small,
    Medium,
    Large,
}

impl Default for CheckboxSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Props for the Checkbox component
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Whether the checkbox is checked
    #[props(default = Signal::new(false))]
    pub checked: Signal<bool>,
    
    /// Callback for when the checkbox is toggled
    #[props(default)]
    pub on_checked_change: Option<EventHandler<bool>>,
    
    /// Whether the checkbox is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,
    
    /// Size of the checkbox
    #[props(default)]
    pub size: CheckboxSize,
    
    /// Optional ID for the checkbox
    #[props(default)]
    pub id: Option<String>,
    
    /// Name attribute for form submission
    #[props(default)]
    pub name: Option<String>,
    
    /// Accessible label for the checkbox
    #[props(default)]
    pub aria_label: Option<String>,
    
    /// Optional children (usually the indicator)
    #[props(default)]
    pub children: Element,
    
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A styled checkbox component that can be toggled on or off
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    // Generate unique ID if not provided
    let checkbox_id = use_unique_id();
    let id = props.id.clone().unwrap_or_else(|| checkbox_id());
    
    // Determine size-specific classes
    let (size_class, icon_size) = match props.size {
        CheckboxSize::Small => ("h-4 w-4", "h-3 w-3"),
        CheckboxSize::Medium => ("h-5 w-5", "h-4 w-4"),
        CheckboxSize::Large => ("h-6 w-6", "h-5 w-5"),
    };
    
    // Build checkbox wrapper classes
    let checkbox_class = format!(
        "inline-flex items-center justify-center rounded border-2 transition-colors {} {} {}",
        size_class,
        if (props.checked)() { "bg-primary border-primary" } else { "bg-background border-input hover:bg-accent/10" },
        if (props.disabled)() { "cursor-not-allowed opacity-50" } else { "cursor-pointer" }
    );
    
    // Handle checkbox change
    let mut checked = props.checked.clone();
    let on_change = move |_| {
        if !(props.disabled)() {
            if let Some(handler) = &props.on_checked_change {
                let new_state = !checked();
                checked.set(new_state);
                handler.call(new_state);
            }
        }
    };
    
    rsx! {
        div {
            class: checkbox_class,
            role: "checkbox",
            "aria-checked": (props.checked)().to_string(),
            id: id.clone(),
            onclick: on_change,
            tabindex: if !(props.disabled)() { "0" } else { "-1" },
            
            // Render indicator when checked
            if (props.checked)() {
                div {
                    class: format!("flex items-center justify-center {}", icon_size),
                    
                    Check {
                        class: "text-primary-foreground"
                    }
                }
            }
            
            // Hidden input for form submission
            if let Some(name) = &props.name {
                input {
                    type: "checkbox",
                    id: format!("{}-input", id),
                    name: name.clone(),
                    checked: (props.checked)(),
                    disabled: (props.disabled)(),
                    class: "sr-only",
                }
            }
        }
    }
}
