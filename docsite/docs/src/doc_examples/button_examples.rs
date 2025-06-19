#![allow(non_snake_case)]

pub use variants::ButtonVariantsExample;
pub use sizes::ButtonSizesExample;
pub use states::ButtonStatesExample;
pub use icons::ButtonWithIconsExample;
pub use full_width::FullWidthButtonExample;
pub use icon_buttons::IconButtonsExample;

pub mod variants {
    // ANCHOR: variants
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant};
    
    #[component]
    pub fn ButtonVariantsExample() -> Element {
        rsx! {
            div { class: "flex flex-wrap gap-2.5 items-center",
                Button {
                    variant: ButtonVariant::Primary,
                    "Primary"
                }
                
                Button {
                    variant: ButtonVariant::Secondary,
                    "Secondary"
                }
                
                Button {
                    variant: ButtonVariant::Outline,
                    "Outline"
                }
                
                Button {
                    variant: ButtonVariant::Ghost,
                    "Ghost"
                }
                
                Button {
                    variant: ButtonVariant::Link,
                    "Link"
                }
                
                Button {
                    variant: ButtonVariant::Destructive,
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
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Small,
                    "Small"
                }
                
                Button {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Medium,
                    "Medium"
                }
                
                Button {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Large,
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
    use laminar_blocks::components::button::{Button, ButtonVariant};
    
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
                    variant: ButtonVariant::Primary,
                    disabled: true,
                    "Disabled"
                }
                
                Button {
                    variant: ButtonVariant::Primary,
                    loading: loading(),
                    "Loading"
                }
                
                Button {
                    variant: ButtonVariant::Secondary,
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
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use lucide_dioxus::{ArrowLeft, ArrowRight};
    
    #[component]
    pub fn ButtonWithIconsExample() -> Element {
        rsx! {
            div { class: "flex flex-wrap gap-2.5 items-center",
                Button {
                    variant: ButtonVariant::Primary,
                    icon_left: rsx! { ArrowLeft { size: 16, color: "currentColor" } },
                    "Left Icon"
                }
                
                Button {
                    variant: ButtonVariant::Primary,
                    icon_right: rsx! { ArrowRight { size: 16, color: "currentColor" } },
                    "Right Icon"
                }
                
                Button {
                    variant: ButtonVariant::Secondary,
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
    use laminar_blocks::components::button::{Button, ButtonVariant};
    
    #[component]
    pub fn FullWidthButtonExample() -> Element {
        rsx! {
            div { class: "w-full",
                Button {
                    variant: ButtonVariant::Primary,
                    full_width: true,
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
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use lucide_dioxus::{Plus, Pencil, Trash, Search, X};
    
    #[component]
    pub fn IconButtonsExample() -> Element {
        rsx! {
            div { class: "flex flex-wrap gap-2.5 items-center",
                Button {
                    variant: ButtonVariant::Primary,
                    is_icon_button: true,
                    aria_label: Some("Add item".to_string()),
                    Plus { size: 20, color: "currentColor" }
                }
                
                Button {
                    variant: ButtonVariant::Secondary,
                    is_icon_button: true,
                    aria_label: Some("Edit item".to_string()),
                    Pencil { size: 20, color: "currentColor" }
                }
                
                Button {
                    variant: ButtonVariant::Outline,
                    is_icon_button: true,
                    aria_label: Some("Delete item".to_string()),
                    Trash { size: 20, color: "currentColor" }
                }
                
                Button {
                    variant: ButtonVariant::Ghost,
                    is_icon_button: true,
                    aria_label: Some("Search".to_string()),
                    Search { size: 20, color: "currentColor" }
                }
                
                Button {
                    variant: ButtonVariant::Destructive,
                    is_icon_button: true,
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
    use laminar_blocks::components::button::{Button, ButtonVariant};
    
    #[component]
    pub fn ButtonExample() -> Element {
            
        rsx! {
            Button {
                variant: ButtonVariant::Outline,
                "Outline"
            }
        }
    }
}
