#![allow(non_snake_case)]


pub use basic::BasicCollapsibleExample;
pub use multiple::MultipleCollapsiblesExample;
pub use nested::NestedCollapsibleExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::collapsible::{
        Collapsible, CollapsibleContent, CollapsibleTrigger,
    };
    
    #[component]
    pub fn BasicCollapsibleExample() -> Element {
        rsx! {
            Collapsible {
                CollapsibleTrigger {
                    "What is a collapsible component?"
                }
                
                CollapsibleContent {
                    div {
                        class: "space-y-3 p-4",
                        p { "A collapsible component allows you to toggle the visibility of content sections." }
                        p { "It's commonly used for FAQs, expandable cards, and progressive disclosure patterns." }
                        p { "This implementation includes smooth animations and follows accessibility best practices." }
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
    use laminar_blocks::components::collapsible::{
        Collapsible, CollapsibleContent, CollapsibleTrigger,
    };
    
    #[component]
    pub fn MultipleCollapsiblesExample() -> Element {
        rsx! {
            div { class: "space-y-4",
                Collapsible {
                    CollapsibleTrigger {
                        "When should I use collapsible components?"
                    }
                    
                    CollapsibleContent {
                        div {
                            class: "space-y-2 p-4",
                            p { "Use collapsible components when you need to:" }
                            ul { class: "list-disc pl-5",
                                li { "Save vertical space on a page with a lot of content" }
                                li { "Organize information into logical sections" }
                                li { "Implement progressive disclosure patterns" }
                                li { "Create FAQ sections or accordions" }
                            }
                        }
                    }
                }
                
                Collapsible {
                    CollapsibleTrigger {
                        "Are collapsible components accessible?"
                    }
                    
                    CollapsibleContent {
                        div {
                            class: "space-y-2 p-4",
                            p { "Yes, our collapsible components follow accessibility best practices:" }
                            ul { class: "list-disc pl-5",
                                li { "Proper ARIA attributes for screen readers" }
                                li { "Keyboard navigation support" }
                                li { "Focus management" }
                                li { "Appropriate color contrast" }
                            }
                        }
                    }
                }
                
                Collapsible {
                    CollapsibleTrigger {
                        "Can I customize the styling?"
                    }
                    
                    CollapsibleContent {
                        div {
                            class: "space-y-2 p-4",
                            p { "Absolutely! The collapsible component is designed to be flexible:" }
                            ul { class: "list-disc pl-5",
                                li { "Apply custom classes to any part of the component" }
                                li { "Override default animations" }
                                li { "Add icons or other elements to triggers" }
                                li { "Nest collapsibles for more complex interfaces" }
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: multiple
}

pub mod nested {
    // ANCHOR: nested
    use dioxus::prelude::*;
    use laminar_blocks::components::collapsible::{
        Collapsible, CollapsibleContent, CollapsibleTrigger,
    };
    
    #[component]
    pub fn NestedCollapsibleExample() -> Element {
        rsx! {
            Collapsible {
                CollapsibleTrigger {
                    "Advanced collapsible patterns"
                }
                
                CollapsibleContent {
                    div {
                        class: "p-4 space-y-4",
                        p { "Collapsibles can be nested to create more complex disclosure patterns:" }
                        
                        // Nested collapsible
                        Collapsible {
                            class: "ml-4 border-l-2 pl-4 border-gray-200",
                            
                            CollapsibleTrigger {
                                class: "text-sm",
                                "Nested collapsible example"
                            }
                            
                            CollapsibleContent {
                                div {
                                    class: "p-3 text-sm",
                                    p { "This is a nested collapsible component." }
                                    p { class: "mt-2", "You can nest these as deeply as needed for your information architecture." }
                                    
                                    // Another level of nesting
                                    Collapsible {
                                        class: "ml-4 border-l-2 pl-4 border-gray-200 mt-3",
                                        
                                        CollapsibleTrigger {
                                            class: "text-xs",
                                            "Even deeper nesting"
                                        }
                                        
                                        CollapsibleContent {
                                            div {
                                                class: "p-2 text-xs",
                                                p { "You can create very complex information hierarchies using nested collapsibles." }
                                                p { class: "mt-1", "Just be mindful of the user experience and avoid excessive nesting." }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: nested
}

// Original example for backward compatibility
pub mod example {
    use dioxus::prelude::*;
    use laminar_blocks::components::collapsible::{
        Collapsible, CollapsibleContent, CollapsibleTrigger,
    };
    
    #[component]
    pub fn CollapsibleExample() -> Element {
        rsx! {
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
