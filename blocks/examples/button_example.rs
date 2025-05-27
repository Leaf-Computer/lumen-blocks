use dioxus::prelude::*;
use dioxus_blocks::components::button::{Button, ButtonVariant, ButtonSize};
use log;
use lucide_dioxus::{ArrowLeft, ArrowRight, RefreshCw, Plus, Pencil, Trash, Search, X};

fn main() {
    // Initialize logger for debug builds
    #[cfg(debug_assertions)]
    {
        // This simple configuration will print to stderr
        // For a real app, consider using env_logger or another implementation
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        
        log::info!("Logger initialized in debug mode");
    }
    
    dioxus::launch(ButtonExample);
}

#[component]
pub fn ButtonExample() -> Element {
// State for loading button
let mut loading = use_signal(|| false);
    
// Toggle loading state
let toggle_loading = move |_| {
    loading.set(!loading());
};
    
rsx! {
    div { class: "button-example p-5 flex flex-col gap-5",
        // Button variants
        div {
            h3 { "Button Variants" }
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
            
        // Button sizes
        div {
            h3 { "Button Sizes" }
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
            
        // Button states
        div {
            h3 { "Button States" }
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
            
        // Buttons with icons
        div {
            h3 { "Buttons with Icons" }
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
            }
        }
            
        // Full width button
        div {
            h3 { "Full Width Button" }
            div { class: "w-full",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    full_width: use_signal(|| true),
                    "Full Width Button"
                }
            }
        }
            
        // Icon buttons
        div {
            h3 { "Icon Buttons" }
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
}
}
