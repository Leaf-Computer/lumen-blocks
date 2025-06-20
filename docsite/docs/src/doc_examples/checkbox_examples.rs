#![allow(non_snake_case)]

pub use basic::BasicCheckboxExample;
pub use sizes::CheckboxSizesExample;
pub use states::CheckboxStatesExample;
pub use form_integration::FormIntegrationExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::checkbox::Checkbox;
    
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
    use laminar_blocks::components::checkbox::Checkbox;
    
    #[component]
    pub fn CheckboxStatesExample() -> Element {
        let checked1 = use_signal(|| false);
        let checked2 = use_signal(|| true);
        
        rsx! {
            div { class: "flex flex-col gap-4",
                div { class: "flex items-center gap-2",
                    Checkbox {
                        checked: checked1,
                        disabled: true,
                        aria_label: Some(String::from("Disabled unchecked checkbox example")),
                    }
                    label { class: "text-sm font-medium text-gray-500",
                        "Disabled Unchecked" 
                    }
                }
                
                div { class: "flex items-center gap-2",
                    Checkbox {
                        checked: checked2,
                        disabled: true,
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

pub mod form_integration {
    // ANCHOR: form_integration
    use dioxus::prelude::*;
    use laminar_blocks::components::{button::Button, checkbox::Checkbox};
    
    #[component]
    pub fn FormIntegrationExample() -> Element {
        let mut terms_accepted = use_signal(|| false);
        let mut newsletter = use_signal(|| true);
        let mut form_status = use_signal(|| "Not submitted".to_string());
        
        rsx! {
            form {
                class: "space-y-4",
                onsubmit: move |e| {
                    // Format and display the form values
                    let new_text = format!("Form submitted with: {:?}", e.values());
                    form_status.set(new_text);
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
                
                Button {
                    button_type: "submit",
                    disabled: !terms_accepted(),
                    "Submit Form"
                }
                
                // Display form submission status
                div { 
                    class: "mt-4 p-2 bg-muted rounded text-sm",
                    code { "{form_status}" }
                }
            }
        }
    }
    // ANCHOR_END: form_integration
}

// This maintains the original example for backward compatibility
pub mod example {
    use dioxus::prelude::*;
    use laminar_blocks::components::checkbox::Checkbox;
    
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
