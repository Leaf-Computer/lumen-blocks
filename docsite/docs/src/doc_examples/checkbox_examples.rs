#![allow(non_snake_case)]

pub use basic::BasicCheckboxExample;
pub use sizes::CheckboxSizesExample;
pub use states::CheckboxStatesExample;
pub use custom_indicator::CustomIndicatorExample;
pub use form_integration::FormIntegrationExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::checkbox::{Checkbox, CheckboxSize};
    
    #[component]
    pub fn BasicCheckboxExample() -> Element {
        let mut checked1 = use_signal(|| false);
        let mut checked2 = use_signal(|| true);
        
        rsx! {
            div { class: "flex flex-col gap-4",
                div { class: "flex items-center gap-2",
                    Checkbox {
                        checked: checked1,
                        on_checked_change: move |new_state| {
                            checked1.set(new_state);
                        },
                        aria_label: Some(String::from("Unchecked example")),
                    }
                    label { class: "text-sm font-medium",
                        "Unchecked by default" 
                    }
                }
                
                div { class: "flex items-center gap-2",
                    Checkbox {
                        checked: checked2,
                        on_checked_change: move |new_state| {
                            checked2.set(new_state);
                        },
                        aria_label: Some(String::from("Checked example")),
                    }
                    label { class: "text-sm font-medium",
                        "Checked by default" 
                    }
                }
                
                div {
                    span { class: "text-xs text-gray-500",
                        if checked1() { "First checkbox state: Checked" } else { "First checkbox state: Unchecked" }
                    }
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod sizes {
    // ANCHOR: sizes
    use dioxus::prelude::*;
    use laminar_blocks::components::checkbox::{Checkbox, CheckboxSize};
    
    #[component]
    pub fn CheckboxSizesExample() -> Element {
        let mut checked1 = use_signal(|| false);
        let mut checked2 = use_signal(|| false);
        let mut checked3 = use_signal(|| false);
        
        rsx! {
            div { class: "flex flex-col gap-4",
                // Small
                div { class: "flex items-center gap-2 py-2",
                    Checkbox {
                        checked: checked1,
                        on_checked_change: move |new_state| {
                            checked1.set(new_state);
                        },
                        size: CheckboxSize::Small,
                        aria_label: Some(String::from("Small checkbox example")),
                    }
                    label { class: "text-sm font-medium",
                        "Small Size" 
                    }
                }
                
                // Medium (default)
                div { class: "flex items-center gap-2 py-2",
                    Checkbox {
                        checked: checked2,
                        on_checked_change: move |new_state| {
                            checked2.set(new_state);
                        },
                        aria_label: Some(String::from("Medium checkbox example")),
                    }
                    label { class: "text-sm font-medium",
                        "Medium Size (Default)" 
                    }
                }
                
                // Large
                div { class: "flex items-center gap-2 py-2",
                    Checkbox {
                        checked: checked3,
                        on_checked_change: move |new_state| {
                            checked3.set(new_state);
                        },
                        size: CheckboxSize::Large,
                        aria_label: Some(String::from("Large checkbox example")),
                    }
                    label { class: "text-sm font-medium",
                        "Large Size" 
                    }
                }
            }
        }
    }
    // ANCHOR_END: sizes
}

pub mod states {
    // ANCHOR: states
    use dioxus::prelude::*;
    use laminar_blocks::components::checkbox::{Checkbox, CheckboxSize};
    
    #[component]
    pub fn CheckboxStatesExample() -> Element {
        let mut checked1 = use_signal(|| false);
        let mut checked2 = use_signal(|| true);
        let disabled = ReadOnlySignal::new(Signal::new(true));
        
        rsx! {
            div { class: "flex flex-col gap-4",
                div { class: "flex items-center gap-2",
                    Checkbox {
                        checked: checked1,
                        disabled: disabled,
                        aria_label: Some(String::from("Disabled unchecked checkbox example")),
                    }
                    label { class: "text-sm font-medium text-gray-500",
                        "Disabled Unchecked" 
                    }
                }
                
                div { class: "flex items-center gap-2",
                    Checkbox {
                        checked: checked2,
                        disabled: disabled,
                        aria_label: Some(String::from("Disabled checked checkbox example")),
                    }
                    label { class: "text-sm font-medium text-gray-500",
                        "Disabled Checked" 
                    }
                }
            }
        }
    }
    // ANCHOR_END: states
}

pub mod custom_indicator {
    // ANCHOR: custom_indicator
    use dioxus::prelude::*;
    use laminar_blocks::components::checkbox::{Checkbox, CheckboxSize};
    
    #[component]
    pub fn CustomIndicatorExample() -> Element {
        let mut checked = use_signal(|| false);
        
        rsx! {
            div { class: "flex flex-col gap-4",
                div { class: "flex items-center gap-2",
                    Checkbox {
                        checked: checked,
                        on_checked_change: move |new_state| {
                            checked.set(new_state);
                        },
                        aria_label: Some(String::from("Custom indicator example")),
                        
                        span { class: "text-sm font-bold text-background", "✓" }
                    }
                    label { class: "text-sm font-medium",
                        "Custom checkmark" 
                    }
                }
                
                div { class: "flex items-center gap-2",
                    Checkbox {
                        checked: checked,
                        on_checked_change: move |new_state| {
                            checked.set(new_state);
                        },
                        aria_label: Some(String::from("Custom emoji indicator example")),
                        
                        span { class: "text-sm text-background", "👍" }
                    }
                    label { class: "text-sm font-medium",
                        "Emoji indicator" 
                    }
                }
            }
        }
    }
    // ANCHOR_END: custom_indicator
}

pub mod form_integration {
    // ANCHOR: form_integration
    use dioxus::prelude::*;
    use laminar_blocks::components::checkbox::{Checkbox, CheckboxSize};
    use laminar_blocks::components::button::{Button, ButtonVariant, ButtonSize};
    
    #[component]
    pub fn FormIntegrationExample() -> Element {
        let mut terms_accepted = use_signal(|| false);
        let mut newsletter = use_signal(|| true);
        
        rsx! {
            form {
                class: "space-y-4",
                onsubmit: move |e| {
                    println!("Form submitted with values: {:?}", e.values());
                },
                
                div { class: "flex items-center gap-2",
                    Checkbox {
                        id: Some("terms-checkbox".to_string()),
                        name: Some("terms-accepted".to_string()),
                        checked: terms_accepted,
                        on_checked_change: move |new_state| {
                            terms_accepted.set(new_state);
                        },
                    }
                    label { r#for: "terms-checkbox", class: "text-sm font-medium",
                        "I agree to the terms and conditions" 
                    }
                }
                
                div { class: "flex items-center gap-2",
                    Checkbox {
                        id: Some("newsletter-checkbox".to_string()),
                        name: Some("newsletter".to_string()),
                        checked: newsletter,
                        on_checked_change: move |new_state| {
                            newsletter.set(new_state);
                        },
                    }
                    label { r#for: "newsletter-checkbox", class: "text-sm font-medium",
                        "Subscribe to newsletter" 
                    }
                }
                
                button {
                    r#type: "submit",
                    class: "px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors",
                    disabled: !terms_accepted(),
                    
                    "Submit Form"
                }
            }
        }
    }
    // ANCHOR_END: form_integration
}

// This maintains the original example for backward compatibility
pub mod example {
    use dioxus::prelude::*;
    use laminar_blocks::components::checkbox::{Checkbox, CheckboxSize};
    
    #[component]
    pub fn CheckboxExample() -> Element {
        let mut checked = use_signal(|| false);
            
        rsx! {
            Checkbox {
                checked: checked,
                on_checked_change: move |new_state| {
                    checked.set(new_state);
                },
                aria_label: Some(String::from("Example checkbox")),
            }
        }
    }
}