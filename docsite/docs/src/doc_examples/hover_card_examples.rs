#![allow(non_snake_case)]

pub use basic::HoverCardBasicExample;
pub use placement::HoverCardPlacementExample;
pub use profile::HoverCardProfileExample;
pub use tooltip::HoverCardTooltipExample;
pub use icon::HoverCardIconExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::hover_card::{
        HoverCard, HoverCardTrigger, HoverCardContent
    };
    
    #[component]
    pub fn HoverCardBasicExample() -> Element {
        rsx! {
            div { class: "flex flex-col items-center p-48",
                HoverCard {
                    HoverCardTrigger {
                        button { 
                            class: "px-4 py-2 rounded-md bg-primary text-primary-foreground hover:bg-primary/90", 
                            "Hover over me" 
                        }
                    }

                    HoverCardContent { 
                        div { class: "p-4 max-w-xs",
                            h4 { class: "text-sm font-semibold mb-2", "Hover Card" }
                            p { class: "text-sm text-muted-foreground", 
                                "This is a basic hover card that appears when you hover over the trigger element."
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod placement {
    // ANCHOR: placement
    use dioxus::prelude::*;
    use laminar_blocks::components::hover_card::{
        HoverCard, HoverCardTrigger, HoverCardContent, HoverCardAlign, HoverCardSide
    };
    
    #[component]
    pub fn HoverCardPlacementExample() -> Element {
        rsx! {
            div { class: "flex flex-col gap-8 p-48",
                // Top placement
                div { class: "flex flex-row gap-8 justify-center",
                    HoverCard {
                        HoverCardTrigger {
                            button { 
                                class: "px-4 py-2 rounded-md bg-secondary text-secondary-foreground", 
                                "Top Start" 
                            }
                        }

                        HoverCardContent { 
                            side: Some(HoverCardSide::Top),
                            align: Some(HoverCardAlign::Start),
                            div { class: "p-2",
                                p { class: "text-sm", "Top Start Placement" }
                            }
                        }
                    }
                    
                    HoverCard {
                        HoverCardTrigger {
                            button { 
                                class: "px-4 py-2 rounded-md bg-secondary text-secondary-foreground", 
                                "Top Center" 
                            }
                        }

                        HoverCardContent { 
                            side: Some(HoverCardSide::Top),
                            align: Some(HoverCardAlign::Center),
                            div { class: "p-2",
                                p { class: "text-sm", "Top Center Placement" }
                            }
                        }
                    }
                    
                    HoverCard {
                        HoverCardTrigger {
                            button { 
                                class: "px-4 py-2 rounded-md bg-secondary text-secondary-foreground", 
                                "Top End" 
                            }
                        }

                        HoverCardContent { 
                            side: Some(HoverCardSide::Top),
                            align: Some(HoverCardAlign::End),
                            div { class: "p-2",
                                p { class: "text-sm", "Top End Placement" }
                            }
                        }
                    }
                }
                
                // Bottom placement
                div { class: "flex flex-row gap-8 justify-center",
                    HoverCard {
                        HoverCardTrigger {
                            button { 
                                class: "px-4 py-2 rounded-md bg-secondary text-secondary-foreground", 
                                "Bottom Start" 
                            }
                        }

                        HoverCardContent { 
                            side: Some(HoverCardSide::Bottom),
                            align: Some(HoverCardAlign::Start),
                            div { class: "p-2",
                                p { class: "text-sm", "Bottom Start Placement" }
                            }
                        }
                    }
                    
                    HoverCard {
                        HoverCardTrigger {
                            button { 
                                class: "px-4 py-2 rounded-md bg-secondary text-secondary-foreground", 
                                "Bottom Center" 
                            }
                        }

                        HoverCardContent { 
                            side: Some(HoverCardSide::Bottom),
                            align: Some(HoverCardAlign::Center),
                            div { class: "p-2",
                                p { class: "text-sm", "Bottom Center Placement" }
                            }
                        }
                    }
                    
                    HoverCard {
                        HoverCardTrigger {
                            button { 
                                class: "px-4 py-2 rounded-md bg-secondary text-secondary-foreground", 
                                "Bottom End" 
                            }
                        }

                        HoverCardContent { 
                            side: Some(HoverCardSide::Bottom),
                            align: Some(HoverCardAlign::End),
                            div { class: "p-2",
                                p { class: "text-sm", "Bottom End Placement" }
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: placement
}

pub mod profile {
    // ANCHOR: profile
    use dioxus::prelude::*;
    use laminar_blocks::components::hover_card::{
        HoverCard, HoverCardTrigger, HoverCardContent, HoverCardAlign, HoverCardSide
    };
    
    #[component]
    pub fn HoverCardProfileExample() -> Element {
        rsx! {
            div { class: "flex justify-center p-48",
                HoverCard {
                    HoverCardTrigger {
                        button { 
                            class: "px-4 py-2 rounded-md bg-primary text-primary-foreground hover:bg-primary/90", 
                            "@johndoe" 
                        }
                    }

                    HoverCardContent { 
                        side: Some(HoverCardSide::Bottom),
                        align: Some(HoverCardAlign::Start),
                        div { class: "user-card",
                            div { class: "flex items-center gap-3 mb-3",
                                img {
                                    class: "w-12 h-12 rounded-full border border-border",
                                    src: "https://github.com/DioxusLabs.png",
                                    alt: "User avatar",
                                }
                                div {
                                    h4 { class: "text-base font-semibold", "John Doe" }
                                    p { class: "text-sm text-muted-foreground", "@johndoe" }
                                }
                            }

                            p { class: "text-sm mb-4 text-muted-foreground",
                                "Software developer passionate about Rust and web technologies. Building awesome UI components with Dioxus."
                            }

                            div { class: "flex justify-between items-center pt-3 border-t border-border",
                                div { class: "text-center",
                                    div { class: "font-medium", "142" }
                                    div { class: "text-xs text-muted-foreground", "Posts" }
                                }
                                div { class: "text-center",
                                    div { class: "font-medium", "2.5k" }
                                    div { class: "text-xs text-muted-foreground", "Followers" }
                                }
                                div { class: "text-center",
                                    div { class: "font-medium", "350" }
                                    div { class: "text-xs text-muted-foreground", "Following" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: profile
}

pub mod tooltip {
    // ANCHOR: tooltip
    use dioxus::prelude::*;
    use laminar_blocks::components::hover_card::{
        HoverCard, HoverCardTrigger, HoverCardContent, HoverCardAlign, HoverCardSide
    };
    use lucide_dioxus::ExternalLink;
    
    #[component]
    pub fn HoverCardTooltipExample() -> Element {
        rsx! {
            div { class: "flex justify-center p-48",
                HoverCard {
                    HoverCardTrigger {
                        a { 
                            class: "text-blue-600 hover:underline flex items-center gap-1", 
                            href: "#", 
                            "Visit documentation site"
                            ExternalLink { size: 16, color: "currentColor" }
                        }
                    }

                    HoverCardContent {
                        side: Some(HoverCardSide::Top),
                        align: Some(HoverCardAlign::Center),
                        class: Some("p-3 max-w-xs".to_string()),
                        div {
                            p { class: "text-sm", "This link will take you to the Dioxus documentation." }
                            div { class: "text-xs text-muted-foreground mt-1", "Click to continue" }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: tooltip
}

pub mod icon {
    // ANCHOR: icon
    use dioxus::prelude::*;
    use laminar_blocks::components::hover_card::{
        HoverCard, HoverCardTrigger, HoverCardContent, HoverCardSide
    };
    use lucide_dioxus::Info;
    
    #[component]
    pub fn HoverCardIconExample() -> Element {
        rsx! {
            div { class: "flex justify-center p-48",
                HoverCard {
                    HoverCardTrigger {
                        button { 
                            class: "w-10 h-10 rounded-full bg-secondary flex items-center justify-center hover:bg-secondary/80", 
                            Info { size: 20, color: "currentColor" }
                        }
                    }

                    HoverCardContent {
                        side: Some(HoverCardSide::Right),
                        class: Some("p-4".to_string()),
                        div {
                            h4 { class: "text-sm font-semibold mb-2", "Additional Information" }
                            p { class: "text-sm text-muted-foreground", 
                                "Hover cards provide contextual information without requiring a click."
                            }
                            p { class: "text-sm text-muted-foreground mt-2", 
                                "Use them to show previews, detailed information, or quick actions."
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: icon
}
