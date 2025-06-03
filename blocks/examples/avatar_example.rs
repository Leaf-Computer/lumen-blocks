use dioxus_lib::prelude::*;
use dioxus_blocks::components::avatar::{Avatar, AvatarFallback, AvatarImage};

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
    
    dioxus::launch(avatar_examples);
}

pub fn avatar_examples() -> Element {
    rsx! {
        div {
            class: "container mx-auto p-8",
            
            h1 { class: "text-2xl font-bold mb-6", "Avatar Examples" }
            
            // Basic examples section
            section {
                class: "mb-10",
                h2 { class: "text-xl font-semibold mb-4", "Basic Avatar" }
                p { class: "mb-4", "Basic avatar with image and fallback." }
                
                div {
                    class: "flex items-center gap-4 p-4 border rounded-lg border-border",
                    BasicAvatarExample {}
                }
            }
            
            // Size variations section
            section {
                class: "mb-10",
                h2 { class: "text-xl font-semibold mb-4", "Size Variations" }
                p { class: "mb-4", "Avatars in different sizes." }
                
                div {
                    class: "flex items-center gap-4 p-4 border rounded-lg border-border",
                    SizeVariationsExample {}
                }
            }
            
            // Error state section
            section {
                class: "mb-10",
                h2 { class: "text-xl font-semibold mb-4", "Error State & Fallbacks" }
                p { class: "mb-4", "Avatars showing fallback content when images fail to load." }
                
                div {
                    class: "flex items-center gap-4 p-4 border rounded-lg border-border",
                    ErrorStateExample {}
                }
            }
            
            // Group avatars section
            section {
                class: "mb-10",
                h2 { class: "text-xl font-semibold mb-4", "Avatar Groups" }
                p { class: "mb-4", "Multiple avatars grouped together." }
                
                div {
                    class: "p-4 border rounded-lg border-border",
                    AvatarGroupExample {}
                }
            }
        }
    }
}

#[component]
fn BasicAvatarExample() -> Element {
    let mut avatar_state = use_signal(|| "No state yet".to_string());

    rsx! {
        div { class: "flex items-center gap-4",
            Avatar {
                on_state_change: move |state| {
                    avatar_state.set(format!("Avatar state: {:?}", state));
                },

                AvatarImage {
                    src: "https://github.com/DioxusLabs.png",
                    alt: "Dioxus Logo",
                }

                AvatarFallback { "DX" }
            }
            
            div { class: "text-sm text-muted-foreground",
                p { "Working avatar with image" }
                p { class: "text-xs", "{avatar_state.read()}" }
            }
        }
    }
}

#[component]
fn SizeVariationsExample() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            // Small avatar
            Avatar {
                class: "h-8 w-8",
                
                AvatarImage {
                    src: "https://github.com/DioxusLabs.png",
                    alt: "Small avatar",
                }
                AvatarFallback { class: "text-xs", "SM" }
            }
            
            // Default/Medium avatar
            Avatar {
                AvatarImage {
                    src: "https://github.com/DioxusLabs.png",
                    alt: "Medium avatar",
                }
                AvatarFallback { "MD" }
            }
            
            // Large avatar
            Avatar {
                class: "h-16 w-16",
                
                AvatarImage {
                    src: "https://github.com/DioxusLabs.png",
                    alt: "Large avatar",
                }
                AvatarFallback { class: "text-lg", "LG" }
            }
            
            // Extra large avatar
            Avatar {
                class: "h-20 w-20",
                
                AvatarImage {
                    src: "https://github.com/DioxusLabs.png",
                    alt: "Extra large avatar",
                }
                AvatarFallback { class: "text-xl", "XL" }
            }
        }
    }
}

#[component]
fn ErrorStateExample() -> Element {
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

#[component]
fn AvatarGroupExample() -> Element {
    rsx! {
        div { class: "space-y-6",
            // Overlapping avatars
            div {
                h3 { class: "text-lg font-medium mb-2", "Overlapping Group" }
                div { class: "flex -space-x-2",
                    Avatar {
                        class: "border-2 border-background",
                        AvatarImage {
                            src: "https://github.com/DioxusLabs.png",
                            alt: "User 1",
                        }
                        AvatarFallback { "U1" }
                    }
                    Avatar {
                        class: "border-2 border-background",
                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "User 2",
                        }
                        AvatarFallback { "U2" }
                    }
                    Avatar {
                        class: "border-2 border-background",
                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
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
                            src: "https://github.com/DioxusLabs.png",
                            alt: "Team member 1",
                        }
                        AvatarFallback { "T1" }
                    }
                    Avatar {
                        class: "h-12 w-12",
                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "Team member 2",
                        }
                        AvatarFallback { "T2" }
                    }
                    Avatar {
                        class: "h-12 w-12",
                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "Team member 3",
                        }
                        AvatarFallback { "T3" }
                    }
                }
            }
        }
    }
}