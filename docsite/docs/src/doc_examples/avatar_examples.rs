#![allow(non_snake_case)]


pub use basic::BasicAvatarExample;
pub use sizes::SizeVariationsExample;
pub use fallbacks::ErrorStateExample;
pub use groups::AvatarGroupExample;
pub use state::AvatarStateExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::avatar::{Avatar, AvatarFallback, AvatarImage};
    
    const PERSON_1: Asset = asset!("/assets/person-1.avif");
    
    #[component]
    pub fn BasicAvatarExample() -> Element {
        rsx! {
            div { class: "flex items-center gap-4",
                Avatar {
                    AvatarImage {
                        src: PERSON_1,
                        alt: "Person",
                    }

                    AvatarFallback { "DX" }
                }
                
                div { class: "text-sm text-muted-foreground",
                    p { "Avatar with image and fallback" }
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod sizes {
    // ANCHOR: sizes
    use dioxus::prelude::*;
    use laminar_blocks::components::avatar::{Avatar, AvatarFallback, AvatarImage};
    
    const PERSON_1: Asset = asset!("/assets/person-1.avif");
    
    #[component]
    pub fn SizeVariationsExample() -> Element {
        rsx! {
            div { class: "flex items-center gap-4",
                // Small avatar
                Avatar {
                    class: "h-8 w-8",
                    
                    AvatarImage {
                        src: PERSON_1,
                        alt: "Small avatar",
                    }
                    AvatarFallback { class: "text-xs", "SM" }
                }
                
                // Default/Medium avatar
                Avatar {
                    AvatarImage {
                        src: PERSON_1,
                        alt: "Medium avatar",
                    }
                    AvatarFallback { "MD" }
                }
                
                // Large avatar
                Avatar {
                    class: "h-16 w-16",
                    
                    AvatarImage {
                        src: PERSON_1,
                        alt: "Large avatar",
                    }
                    AvatarFallback { class: "text-lg", "LG" }
                }
                
                // Extra large avatar
                Avatar {
                    class: "h-20 w-20",
                    
                    AvatarImage {
                        src: PERSON_1,
                        alt: "Extra large avatar",
                    }
                    AvatarFallback { class: "text-xl", "XL" }
                }
            }
        }
    }
    // ANCHOR_END: sizes
}

pub mod fallbacks {
    // ANCHOR: fallbacks
    use dioxus::prelude::*;
    use laminar_blocks::components::avatar::{Avatar, AvatarFallback, AvatarImage};
    
    #[component]
    pub fn ErrorStateExample() -> Element {
        rsx! {
            div { class: "flex items-center gap-4",
                // Avatar with invalid image (shows text fallback)
                Avatar {
                    AvatarImage {
                        src: "https://invalid-url.example/image.jpg",
                        alt: "Invalid image",
                    }
                    AvatarFallback { "JD" }
                }
                
                // Avatar with emoji fallback
                Avatar {
                    AvatarImage {
                        src: "https://invalid-url.example/image.jpg",
                        alt: "Invalid image",
                    }
                    AvatarFallback { "👤" }
                }
                
                // Avatar with icon-style fallback
                Avatar {
                    AvatarImage {
                        src: "https://invalid-url.example/image.jpg",
                        alt: "Invalid image",  
                    }
                    AvatarFallback { "?" }
                }
                
                // Avatar with colorful fallback
                Avatar {
                    AvatarImage {
                        src: "https://invalid-url.example/image.jpg",
                        alt: "Invalid image",
                    }
                    AvatarFallback { 
                        class: "bg-blue-500 text-white",
                        "AB" 
                    }
                }
            }
        }
    }
    // ANCHOR_END: fallbacks
}

pub mod groups {
    // ANCHOR: groups
    use dioxus::prelude::*;
    use laminar_blocks::components::avatar::{Avatar, AvatarFallback, AvatarImage};
    
    const PERSON_1: Asset = asset!("/assets/person-1.avif");
    const PERSON_2: Asset = asset!("/assets/person-2.jpeg");
    const PERSON_3: Asset = asset!("/assets/person-3.avif");
    
    #[component]
    pub fn AvatarGroupExample() -> Element {
        rsx! {
            div { class: "space-y-6",
                // Overlapping avatars
                div {
                    h3 { class: "text-lg font-medium mb-2", "Overlapping Group" }
                    div { class: "flex -space-x-2",
                        Avatar {
                            class: "border-2 border-background",
                            AvatarImage {
                                src: PERSON_1,
                                alt: "User 1",
                            }
                            AvatarFallback { "U1" }
                        }
                        Avatar {
                            class: "border-2 border-background",
                            AvatarImage {
                                src: PERSON_2,
                                alt: "User 2",
                            }
                            AvatarFallback { "U2" }
                        }
                        Avatar {
                            class: "border-2 border-background",
                            AvatarImage {
                                src: PERSON_3,
                                alt: "User 3",
                            }
                            AvatarFallback { "U3" }
                        }
                        Avatar {
                            class: "border-2 border-background bg-muted",
                            AvatarFallback { "+2" }
                        }
                    }
                }
                
                // Spaced group
                div {
                    h3 { class: "text-lg font-medium mb-2", "Spaced Group" }
                    div { class: "flex gap-2",
                        Avatar {
                            class: "h-12 w-12",
                            AvatarImage {
                                src: PERSON_1,
                                alt: "Team member 1",
                            }
                            AvatarFallback { "T1" }
                        }
                        Avatar {
                            class: "h-12 w-12",
                            AvatarImage {
                                src: PERSON_2,
                                alt: "Team member 2",
                            }
                            AvatarFallback { "T2" }
                        }
                        Avatar {
                            class: "h-12 w-12",
                            AvatarImage {
                                src: PERSON_3,
                                alt: "Team member 3",
                            }
                            AvatarFallback { "T3" }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: groups
}

pub mod state {
    // ANCHOR: state
    use dioxus::prelude::*;
    use laminar_blocks::components::avatar::{Avatar, AvatarFallback, AvatarImage};
    
    const PERSON_1: Asset = asset!("/assets/person-1.avif");
    
    #[component]
    pub fn AvatarStateExample() -> Element {
        let mut avatar_state = use_signal(|| "No state yet".to_string());

        rsx! {
            div { class: "space-y-4",
                div { class: "flex items-center gap-4",
                    Avatar {
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar state: {:?}", state));
                        },

                        AvatarImage {
                            src: PERSON_1,
                            alt: "Person 1",
                        }

                        AvatarFallback { "DX" }
                    }
                    
                    div { class: "text-sm",
                        p { "Working avatar with state tracking" }
                    }
                }
                
                div { class: "p-2 bg-muted rounded text-sm",
                    code { "{avatar_state}" }
                }
                
                div { class: "flex items-center gap-4",
                    Avatar {
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar state: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "Invalid image",
                            },

                            AvatarFallback { "ER" }
                    }
                    
                    div { class: "text-sm",
                        p { "Avatar with invalid image (triggers error state)" }
                    }
                }
            }
        }
    }
    // ANCHOR_END: state
}
