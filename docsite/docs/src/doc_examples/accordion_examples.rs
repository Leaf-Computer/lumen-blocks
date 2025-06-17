#![allow(non_snake_case)]

use dioxus::prelude::*;
use laminar_blocks::components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

pub use basic::BasicAccordionExample;
pub use multiple::MultipleOpenAccordionExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
    
    #[component]
    pub fn BasicAccordionExample() -> Element {
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
    // ANCHOR_END: basic
}

pub mod multiple {
    // ANCHOR: multiple
    use dioxus::prelude::*;
    use laminar_blocks::components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
    
    #[component]
    pub fn MultipleOpenAccordionExample() -> Element {
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
    // ANCHOR_END: multiple
}
