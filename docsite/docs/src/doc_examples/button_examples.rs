#![allow(non_snake_case)]

use lucide_dioxus::{ArrowLeft, ArrowRight, RefreshCw, Plus, Pencil, Trash, Search, X};

pub use variants::ButtonVariantsExample;
pub use sizes::ButtonSizesExample;
pub use states::ButtonStatesExample;
pub use icons::ButtonWithIconsExample;
pub use full_width::FullWidthButtonExample;
pub use icon_buttons::IconButtonsExample;

pub mod variants {
    // ANCHOR: variants
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant, ButtonSize};
    
    #[component]
    pub fn ButtonVariantsExample() -> Element {
        rsx! {
            div { class: "flex flex-wrap gap-2.5 items-center",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    "Primary"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Secondary),
                    "Secondary"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Outline),
                    "Outline"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Ghost),
                    "Ghost"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Link),
                    "Link"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Destructive),
                    "Destructive"
                }
            }
        }
    }
    // ANCHOR_END: variants
}

pub mod sizes {
    // ANCHOR: sizes
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant, ButtonSize};
    
    #[component]
    pub fn ButtonSizesExample() -> Element {
        rsx! {
            div { class: "flex flex-wrap gap-2.5 items-center",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    size: use_signal(|| ButtonSize::Small),
                    "Small"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    size: use_signal(|| ButtonSize::Medium),
                    "Medium"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    size: use_signal(|| ButtonSize::Large),
                    "Large"
                }
            }
        }
    }
    // ANCHOR_END: sizes
}

pub mod states {
    // ANCHOR: states
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant, ButtonSize};
    
    #[component]
    pub fn ButtonStatesExample() -> Element {
        // State for loading button
        let mut loading = use_signal(|| false);
            
        // Toggle loading state
        let toggle_loading = move |_| {
            loading.set(!loading());
        };
        
        rsx! {
            div { class: "flex flex-wrap gap-2.5 items-center",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    disabled: use_signal(|| true),
                    "Disabled"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    loading: loading,
                    "Loading"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Secondary),
                    on_click: toggle_loading,
                    "Toggle Loading"
                }
            }
        }
    }
    // ANCHOR_END: states
}

pub mod icons {
    // ANCHOR: icons
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant, ButtonSize};
    use lucide_dioxus::{ArrowLeft, ArrowRight};
    
    #[component]
    pub fn ButtonWithIconsExample() -> Element {
        rsx! {
            div { class: "flex flex-wrap gap-2.5 items-center",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    icon_left: rsx! { ArrowLeft { size: 16, color: "currentColor" } },
                    "Left Icon"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    icon_right: rsx! { ArrowRight { size: 16, color: "currentColor" } },
                    "Right Icon"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Secondary),
                    icon_left: rsx! { ArrowLeft { size: 16, color: "currentColor" } },
                    icon_right: rsx! { ArrowRight { size: 16, color: "currentColor" } },
                    "Both Icons"
                }
            }
        }
    }
    // ANCHOR_END: icons
}

pub mod full_width {
    // ANCHOR: full_width
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant, ButtonSize};
    
    #[component]
    pub fn FullWidthButtonExample() -> Element {
        rsx! {
            div { class: "w-full",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    full_width: use_signal(|| true),
                    "Full Width Button"
                }
            }
        }
    }
    // ANCHOR_END: full_width
}

pub mod icon_buttons {
    // ANCHOR: icon_buttons
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant, ButtonSize};
    use lucide_dioxus::{Plus, Pencil, Trash, Search, X};
    
    #[component]
    pub fn IconButtonsExample() -> Element {
        rsx! {
            div { class: "flex flex-wrap gap-2.5 items-center",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Add item".to_string()),
                    Plus { size: 20, color: "currentColor" }
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Secondary),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Edit item".to_string()),
                    Pencil { size: 20, color: "currentColor" }
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Outline),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Delete item".to_string()),
                    Trash { size: 20, color: "currentColor" }
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Ghost),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Search".to_string()),
                    Search { size: 20, color: "currentColor" }
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Destructive),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Close".to_string()),
                    X { size: 20, color: "currentColor" }
                }
            }
        }
    }
    // ANCHOR_END: icon_buttons
}

// This maintains the original example for backward compatibility
pub mod example {
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant, ButtonSize};
    
    #[component]
    pub fn ButtonExample() -> Element {
        // State for loading button
        let mut loading = use_signal(|| false);
            
        // Toggle loading state
        let toggle_loading = move |_: String| {
            loading.set(!loading());
        };
            
        rsx! {
            Button {
                variant: use_signal(|| ButtonVariant::Outline),
                "Outline"
            }
        }
    }
}
