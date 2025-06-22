#![allow(non_snake_case)]

pub use basic::BasicSwitchExample;
pub use sizes::SwitchSizesExample;
pub use states::SwitchStatesExample;
pub use with_text::SwitchWithTextExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::switch::Switch;
    
    #[component]
    pub fn BasicSwitchExample() -> Element {
        let mut checked = use_signal(|| false);
        
        rsx! {
            div { class: "flex items-center justify-between py-3",
                label { class: "text-sm font-medium",
                    "Toggle me"
                }
                Switch {
                    checked: checked,
                    on_checked_change: move |new_state| {
                        checked.set(new_state);
                    },
                    aria_label: Some(String::from("Toggle basic example")),
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod sizes {
    // ANCHOR: sizes
    use dioxus::prelude::*;
    use laminar_blocks::components::switch::{Switch, SwitchSize};
    
    #[component]
    pub fn SwitchSizesExample() -> Element {
        let mut small_checked = use_signal(|| false);
        let mut medium_checked = use_signal(|| true);
        let mut large_checked = use_signal(|| false);
        
        rsx! {
            div { class: "space-y-4",
                // Small
                div { class: "flex items-center justify-between py-2",
                    label { class: "text-sm font-medium",
                        "Small Size" 
                    }
                    Switch {
                        checked: small_checked,
                        on_checked_change: move |new_state| {
                            small_checked.set(new_state);
                        },
                        size: SwitchSize::Small,
                        aria_label: Some(String::from("Small switch example")),
                    }
                }
                
                // Medium (default)
                div { class: "flex items-center justify-between py-2",
                    label { class: "text-sm font-medium",
                        "Medium Size (Default)" 
                    }
                    Switch {
                        checked: medium_checked,
                        on_checked_change: move |new_state| {
                            medium_checked.set(new_state);
                        },
                        aria_label: Some(String::from("Medium switch example")),
                    }
                }
                
                // Large
                div { class: "flex items-center justify-between py-2",
                    label { class: "text-sm font-medium",
                        "Large Size" 
                    }
                    Switch {
                        checked: large_checked,
                        on_checked_change: move |new_state| {
                            large_checked.set(new_state);
                        },
                        size: SwitchSize::Large,
                        aria_label: Some(String::from("Large switch example")),
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
    use laminar_blocks::components::switch::Switch;
    
    #[component]
    pub fn SwitchStatesExample() -> Element {
        let mut checked_enabled = use_signal(|| true);
        let checked_disabled = use_signal(|| true);
        let unchecked_disabled = use_signal(|| false);
        
        let disabled = ReadOnlySignal::new(Signal::new(true));
        
        rsx! {
            div { class: "space-y-4",
                div { class: "flex items-center justify-between py-2",
                    label { class: "text-sm font-medium",
                        "Enabled Switch" 
                    }
                    Switch {
                        checked: checked_enabled,
                        on_checked_change: move |new_state| {
                            checked_enabled.set(new_state);
                        },
                        aria_label: Some(String::from("Enabled switch example")),
                    }
                }
                
                div { class: "flex items-center justify-between py-2",
                    label { class: "text-sm font-medium text-gray-500",
                        "Disabled Switch (On)" 
                    }
                    Switch {
                        checked: checked_disabled,
                        disabled: disabled,
                        aria_label: Some(String::from("Disabled switch (on) example")),
                    }
                }
                
                div { class: "flex items-center justify-between py-2",
                    label { class: "text-sm font-medium text-gray-500",
                        "Disabled Switch (Off)" 
                    }
                    Switch {
                        checked: unchecked_disabled,
                        disabled: disabled,
                        aria_label: Some(String::from("Disabled switch (off) example")),
                    }
                }
            }
        }
    }
    // ANCHOR_END: states
}

pub mod with_text {
    // ANCHOR: with_text
    use dioxus::prelude::*;
    use laminar_blocks::components::switch::Switch;
    
    #[component]
    pub fn SwitchWithTextExample() -> Element {
        let mut checked = use_signal(|| true);
        
        rsx! {
            div { class: "flex items-center justify-between py-3 gap-2",
                div {
                    label { class: "text-sm font-medium block",
                        "Airplane Mode" 
                    }
                    span { class: "text-xs text-gray-500",
                        if checked() { "On - Wireless communications are disabled" } 
                        else { "Off - Wireless communications are enabled" }
                    }
                }
                Switch {
                    checked: checked,
                    on_checked_change: move |new_state| {
                        checked.set(new_state);
                    },
                    aria_label: Some(String::from("Toggle airplane mode")),
                }
            }
        }
    }
    // ANCHOR_END: with_text
}
