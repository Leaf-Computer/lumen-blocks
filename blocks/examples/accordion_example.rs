use dioxus_lib::prelude::*;
use dioxus_blocks::components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

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
    
    dioxus::launch(accordion_examples);
}

pub fn accordion_examples() -> Element {
    rsx! {
        div {
            class: "container mx-auto p-8",
            
            h1 { class: "text-2xl font-bold mb-6", "Accordion Examples" }
            
            // Basic example
            section {
                class: "mb-10",
                h2 { class: "text-xl font-semibold mb-4", "Basic Accordion" }
                p { class: "mb-4", "Default accordion with single open item at a time." }
                
                div {
                    class: "max-w-md border rounded-lg border-border",
                    BasicAccordionExample {}
                }
            }
            
            // Multiple open items example
            section {
                class: "mb-10",
                h2 { class: "text-xl font-semibold mb-4", "Multiple Open Items" }
                p { class: "mb-4", "Accordion that allows multiple items to be open simultaneously." }
                
                div {
                    class: "max-w-md border rounded-lg border-border",
                    MultipleOpenAccordionExample {}
                }
            }
        }
    }
}

#[component]
fn BasicAccordionExample() -> Element {
    rsx! {
        Accordion {
            allow_multiple_open: false,
            horizontal: false,

            AccordionItem {
                index: 0,
                AccordionTrigger { "What is Dioxus?" }
                AccordionContent {
                    p { 
                        "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust."
                    }
                }
            }

            AccordionItem {
                index: 1,
                AccordionTrigger { "How does it compare to React?" }
                AccordionContent {
                    p { 
                        "Dioxus is heavily inspired by React but built from the ground up in Rust. It offers similar component-based architecture with hooks, but with the safety and performance benefits of Rust."
                    }
                }
            }

            AccordionItem {
                index: 2,
                AccordionTrigger { "What platforms does it support?" }
                AccordionContent {
                    p { 
                        "Dioxus supports multiple platforms including Web, Desktop (Windows, macOS, Linux), Mobile (iOS, Android), and TUI (Terminal UI)."
                    }
                }
            }
        }
    }
}

#[component]
fn MultipleOpenAccordionExample() -> Element {
    rsx! {
        Accordion {
            allow_multiple_open: true,
            horizontal: false,

            AccordionItem {
                index: 0,
                AccordionTrigger { "First Section" }
                AccordionContent {
                    p { "This is the content for the first section. Multiple sections can be open at once." }
                }
            }

            AccordionItem {
                index: 1,
                AccordionTrigger { "Second Section" }
                AccordionContent {
                    p { "This is the content for the second section. Try opening this while the first section is already open." }
                }
            }

            AccordionItem {
                index: 2,
                AccordionTrigger { "Third Section" }
                AccordionContent {
                    p { "This is the content for the third section. All sections can be open simultaneously." }
                }
            }
        }
    }
}
