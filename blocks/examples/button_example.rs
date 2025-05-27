use dioxus::prelude::*;
use dioxus_blocks::components::button::{Button, ButtonVariant, ButtonSize};
use log;

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
    div { class: "button-example", style: "padding: 20px; display: flex; flex-direction: column; gap: 20px;",
        // Button variants
        div {
            h3 { "Button Variants" }
            div { style: "display: flex; flex-wrap: wrap; gap: 10px; align-items: center;",
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
            div { style: "display: flex; flex-wrap: wrap; gap: 10px; align-items: center;",
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
            div { style: "display: flex; flex-wrap: wrap; gap: 10px; align-items: center;",
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
            div { style: "display: flex; flex-wrap: wrap; gap: 10px; align-items: center;",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    icon_left: rsx! { span { "👈" } },
                    "Left Icon"
                }
                    
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    icon_right: rsx! { span { "👉" } },
                    "Right Icon"
                }
                    
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    icon_left: rsx! { span { "🔄" } },
                    icon_right: rsx! { span { "🔄" } },
                    "Both Icons"
                }
            }
        }
            
        // Full width button
        div {
            h3 { "Full Width Button" }
            div { style: "width: 100%;",
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
            div { style: "display: flex; flex-wrap: wrap; gap: 10px; align-items: center;",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Add item".to_string()),
                    span { "+" }
                }
                    
                Button {
                    variant: use_signal(|| ButtonVariant::Secondary),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Edit item".to_string()),
                    span { "✏️" }
                }
                    
                Button {
                    variant: use_signal(|| ButtonVariant::Outline),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Delete item".to_string()),
                    span { "🗑️" }
                }
                    
                Button {
                    variant: use_signal(|| ButtonVariant::Ghost),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Search".to_string()),
                    span { "🔍" }
                }
                    
                Button {
                    variant: use_signal(|| ButtonVariant::Destructive),
                    is_icon_button: use_signal(|| true),
                    aria_label: Some("Close".to_string()),
                    span { "❌" }
                }
            }
        }
    }
}
}
