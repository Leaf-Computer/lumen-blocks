use dioxus::prelude::*;
use dioxus_blocks::components::checkbox::{Checkbox, CheckboxSize};

fn main() {
    dioxus::launch(CheckboxExample);
}

#[component]
pub fn CheckboxExample() -> Element {
    let mut checked1 = use_signal(|| false);
    let mut checked2 = use_signal(|| true);
    let mut checked3 = use_signal(|| false);
    let mut checked4 = use_signal(|| true);
    let mut checked5 = use_signal(|| false);
    let mut checked6 = use_signal(|| true);
    
    let disabled = ReadOnlySignal::new(Signal::new(true));
    
    rsx! {
        div { class: "p-6 my-6 max-w-md mx-auto bg-background rounded-xl shadow-md text-foreground border border-border",
            h2 { class: "text-xl font-bold mb-4",
                "Checkbox Examples"
            }
            
            // Basic Checkbox
            div { class: "flex items-center justify-between py-3 border-b",
                label { class: "text-sm font-medium",
                    "Basic checkbox"
                }
                div {
                    Checkbox {
                        checked: checked1,
                        on_checked_change: move |new_state| {
                            checked1.set(new_state);
                        },
                        aria_label: Some(String::from("Basic checkbox example")),
                    }
                }
            }
            
            // With Status Text
            div { class: "flex items-center justify-between py-3 border-b",
                div {
                    label { class: "text-sm font-medium block",
                        "Terms and Conditions" 
                    }
                    span { class: "text-xs text-gray-500",
                        if checked2() { "Accepted" } else { "Please accept terms to continue" }
                    }
                }
                div {
                    Checkbox {
                        checked: checked2,
                        on_checked_change: move |new_state| {
                            checked2.set(new_state);
                        },
                        aria_label: Some(String::from("Terms and conditions checkbox")),
                    }
                }
            }
            
            // Different Sizes
            h3 { class: "text-lg font-semibold mt-6 mb-3",
                "Sizes"
            }
            
            div { class: "space-y-4",
                // Small
                div { class: "flex items-center gap-2 py-2",
                    Checkbox {
                        checked: checked3,
                        on_checked_change: move |new_state| {
                            checked3.set(new_state);
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
                        checked: checked4,
                        on_checked_change: move |new_state| {
                            checked4.set(new_state);
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
                        checked: checked5,
                        on_checked_change: move |new_state| {
                            checked5.set(new_state);
                        },
                        size: CheckboxSize::Large,
                        aria_label: Some(String::from("Large checkbox example")),
                    }
                    label { class: "text-sm font-medium",
                        "Large Size" 
                    }
                }
            }
            
            // Custom Indicator
            h3 { class: "text-lg font-semibold mt-6 mb-3",
                "Custom Indicator"
            }
            
            div { class: "flex items-center gap-2 py-2",
                Checkbox {
                    checked: checked1,
                    on_checked_change: move |new_state| {
                        checked1.set(new_state);
                    },
                    aria_label: Some(String::from("Custom indicator example")),
                    
                    span { class: "text-sm font-bold text-background", "✓" }
                }
                label { class: "text-sm font-medium",
                    "Custom checkmark" 
                }
            }
            
            // Disabled State
            h3 { class: "text-lg font-semibold mt-6 mb-3",
                "Disabled State"
            }
            
            div { class: "flex items-center gap-2 py-2",
                Checkbox {
                    checked: checked6,
                    disabled: disabled,
                    aria_label: Some(String::from("Disabled checkbox example")),
                }
                label { class: "text-sm font-medium text-gray-500",
                    "Disabled Checkbox" 
                }
            }
            
            // Form Example
            h3 { class: "text-lg font-semibold mt-6 mb-3",
                "Form Integration"
            }
            
            form {
                class: "space-y-4",
                onsubmit: move |e| {
                    println!("Form submitted with values: {:?}", e.values());
                },
                
                div { class: "flex items-center gap-2",
                    Checkbox {
                        id: Some("terms-checkbox".to_string()),
                        name: Some("terms-accepted".to_string()),
                        checked: checked2,
                        on_checked_change: move |new_state| {
                            checked2.set(new_state);
                        },
                    }
                    label { r#for: "terms-checkbox", class: "text-sm font-medium",
                        "I agree to the terms and conditions" 
                    }
                }
                
                button {
                    r#type: "submit",
                    class: "px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors",
                    disabled: !checked2(),
                    
                    "Submit Form"
                }
            }
        }
    }
}