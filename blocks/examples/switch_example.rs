use dioxus::prelude::*;
use dioxus_blocks::components::switch::{Switch, SwitchSize};

fn main() {
    dioxus::launch(SwitchExample);
}

#[component]
pub fn SwitchExample() -> Element {
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
                "Switch Examples"
            }
            
            // Basic Switch
            div { class: "flex items-center justify-between py-3 border-b",
                label { class: "text-sm font-medium ",
                    "Toggle me"
                }
                Switch {
                    checked: checked1,
                    on_checked_change: move |new_state| {
                        checked1.set(new_state);
                    },
                    aria_label: Some(String::from("Toggle basic example")),
                }
            }
            
            // With Status Text
            div { class: "flex items-center justify-between py-3 border-b",
                div {
                    label { class: "text-sm font-medium  block",
                        "Airplane Mode" 
                    }
                    span { class: "text-xs text-gray-500",
                        if checked2() { "On - All wireless communications are disabled" } 
                        else { "Off - Wireless communications are enabled" }
                    }
                }
                Switch {
                    checked: checked2,
                    on_checked_change: move |new_state| {
                        checked2.set(new_state);
                    },
                    aria_label: Some(String::from("Toggle airplane mode")),
                }
            }
            
            // Different Sizes
            h3 { class: "text-lg font-semibold mt-6 mb-3",
                "Sizes"
            }
            
            div { class: "space-y-4",
                // Small
                div { class: "flex items-center justify-between py-2",
                    label { class: "text-sm font-medium ",
                        "Small Size" 
                    }
                    Switch {
                        checked: checked3,
                        on_checked_change: move |new_state| {
                            checked3.set(new_state);
                        },
                        size: SwitchSize::Small,
                        aria_label: Some(String::from("Small switch example")),
                    }
                }
                
                // Medium (default)
                div { class: "flex items-center justify-between py-2",
                    label { class: "text-sm font-medium ",
                        "Medium Size (Default)" 
                    }
                    Switch {
                        checked: checked4,
                        on_checked_change: move |new_state| {
                            checked4.set(new_state);
                        },
                        aria_label: Some(String::from("Medium switch example")),
                    }
                }
                
                // Large
                div { class: "flex items-center justify-between py-2",
                    label { class: "text-sm font-medium ",
                        "Large Size" 
                    }
                    Switch {
                        checked: checked5,
                        on_checked_change: move |new_state| {
                            checked5.set(new_state);
                        },
                        size: SwitchSize::Large,
                        aria_label: Some(String::from("Large switch example")),
                    }
                }
            }
            
            // Disabled State
            h3 { class: "text-lg font-semibold mt-6 mb-3",
                "Disabled State"
            }
            
            div { class: "flex items-center justify-between py-2",
                label { class: "text-sm font-medium text-gray-500",
                    "Disabled Switch" 
                }
                Switch {
                    checked: checked6,
                    disabled: disabled,
                    aria_label: Some(String::from("Disabled switch example")),
                }
            }
        }
    }
}
