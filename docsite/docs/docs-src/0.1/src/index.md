# Introduction

Welcome to the Laminar Blocks documentation!!

```rust
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
```

```rust, no_run
{{#include src/doc_examples/button_examples.rs:example}}
```
```inject-dioxus
DemoFrame {
    button_examples::example::ButtonExample {}
}
```
