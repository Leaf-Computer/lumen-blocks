use dioxus::prelude::*;
use laminar_blocks::components::hover_card::{
    HoverCard, HoverCardTrigger, HoverCardContent, HoverCardAlign, HoverCardSide
};

fn main() {
    #[cfg(debug_assertions)]
    {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        log::info!("Logger initialized in debug mode");
    }

    dioxus::launch(HoverCardExample);
}

#[component]
pub fn HoverCardExample() -> Element {
    rsx! {
        div { class: "hover-card-example p-12 flex flex-col gap-12 bg-background min-h-[300px]",
            h2 { class: "mb-8 text-2xl font-semibold", "Hover Card Example" }

            div { class: "flex flex-row gap-16 items-center",
                // User profile hover card
                div { class: "flex flex-col items-start",
                    h3 { class: "mb-4 text-lg font-medium", "User Profile Card" }
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

                // Simple link hover card
                div { class: "flex flex-col items-start",
                    h3 { class: "mb-4 text-lg font-medium", "Simple Link Tooltip" }
                    HoverCard {
                        HoverCardTrigger {
                            a { 
                                class: "text-blue-600 hover:underline", 
                                href: "#", 
                                "Hover over this link" 
                            }
                        }

                        HoverCardContent {
                            side: Some(HoverCardSide::Top),
                            align: Some(HoverCardAlign::Center),
                            class: Some("p-3 max-w-xs".to_string()),
                            div {
                                p { class: "text-sm", "This link will take you to an external website." }
                                div { class: "text-xs text-muted-foreground mt-1", "Click to continue" }
                            }
                        }
                    }
                }

                // Icon hover card
                div { class: "flex flex-col items-start",
                    h3 { class: "mb-4 text-lg font-medium", "Icon Info Card" }
                    HoverCard {
                        HoverCardTrigger {
                            button { 
                                class: "w-10 h-10 rounded-full bg-secondary flex items-center justify-center hover:bg-secondary/80", 
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    class: "w-5 h-5",
                                    path { d: "M12 16v-4M12 8h.01" }
                                    circle { cx: "12", cy: "12", r: "10" }
                                }
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

            div { class: "mt-10 p-4 rounded-md bg-muted", 
                p { class: "text-sm text-muted-foreground", 
                    "Hover over each element to see the hover card in action. These cards are fully styled with Tailwind CSS." 
                }
            }
        }
    }
}
