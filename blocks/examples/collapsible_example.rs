use dioxus::prelude::*;
use dioxus_blocks::components::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleTrigger,
};

fn main() {
    #[cfg(debug_assertions)]
    {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        log::info!("Logger initialized in debug mode");
    }

    dioxus::launch(CollapsibleExample);
}

#[component]
pub fn CollapsibleExample() -> Element {
    rsx! {
        div {
            class: "container mx-auto p-8",
            
            h1 { class: "text-3xl font-bold mb-8", "Collapsible Component Examples" }
            
            p { 
                class: "text-gray-600 mb-8",
                "Interactive collapsible components with clean styling inspired by shadcn/ui. Click the triggers to expand and collapse content."
            }
            
            // Basic collapsible example
            section {
                class: "mb-12",
                h2 { class: "text-xl font-semibold mb-4", "Basic Collapsible" }
                p { class: "mb-6 text-gray-600", "A simple collapsible with clean styling and smooth animations." }

                Collapsible {
                    CollapsibleTrigger {
                        "What is a collapsible component?"
                    }
                    
                    CollapsibleContent {
                        div {
                            class: "space-y-3",
                            p { "A collapsible component allows you to toggle the visibility of content sections." }
                            p { "It's commonly used for FAQs, expandable cards, and progressive disclosure patterns." }
                            p { "This implementation includes smooth animations and follows accessibility best practices." }
                        }
                    }
                }
            }
        }
    }
}
