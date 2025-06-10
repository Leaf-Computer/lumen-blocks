#![allow(non_snake_case)]

use lucide_dioxus::{ArrowLeft, ArrowRight, RefreshCw, Plus, Pencil, Trash, Search, X};

pub use example::ButtonExample;

pub mod example {
    // ANCHOR: example
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
    // ANCHOR_END: example
}
