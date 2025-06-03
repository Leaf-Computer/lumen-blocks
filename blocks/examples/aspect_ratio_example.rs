use dioxus::prelude::*;
use dioxus_blocks::components::aspect_ratio::AspectRatio;

fn main() {
    dioxus::launch(AspectRatioExample);
}

#[component]
pub fn AspectRatioExample() -> Element {
    rsx! {
        div { class: "p-6 my-6 max-w-4xl mx-auto bg-background rounded-xl shadow-md text-foreground border border-border",
            h2 { class: "text-2xl font-bold mb-6",
                "Aspect Ratio Examples"
            }
            
            // 16:9 Video Aspect Ratio
            div { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-3",
                    "16:9 Video Aspect Ratio"
                }
                div { class: "max-w-md",
                    AspectRatio { 
                        ratio: 16.0 / 9.0,
                        class: "bg-muted rounded-lg",
                        img {
                            class: "h-full w-full rounded-lg object-cover",
                            src: "https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80",
                            alt: "Photo by Drew Beamer",
                        }
                    }
                }
                p { class: "text-sm text-muted-foreground mt-2",
                    "Perfect for video content and modern displays"
                }
            }
            
            // 4:3 Classic Photo
            div { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-3",
                    "4:3 Classic Photo"
                }
                div { class: "max-w-sm",
                    AspectRatio { 
                        ratio: 4.0 / 3.0,
                        class: "bg-muted rounded-lg",
                        img {
                            class: "h-full w-full rounded-lg object-cover",
                            src: "https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=800&dpr=2&q=80",
                            alt: "Mountain landscape",
                        }
                    }
                }
                p { class: "text-sm text-muted-foreground mt-2",
                    "Traditional photo format"
                }
            }
            
            // 1:1 Square
            div { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-3",
                    "1:1 Square Format"
                }
                div { class: "max-w-xs",
                    AspectRatio { 
                        ratio: 1.0,
                        class: "bg-gradient-to-br from-primary/20 to-secondary/20 rounded-xl border border-border",
                        div { class: "h-full w-full flex items-center justify-center rounded-xl",
                            div { class: "text-center p-4",
                                div { class: "w-12 h-12 bg-primary rounded-full mx-auto mb-3 flex items-center justify-center",
                                    span { class: "text-primary-foreground font-bold text-lg", "📷" }
                                }
                                h4 { class: "font-semibold text-foreground", "Profile Photo" }
                                p { class: "text-sm text-muted-foreground mt-1", "Perfect square format" }
                            }
                        }
                    }
                }
                p { class: "text-sm text-muted-foreground mt-2",
                    "Ideal for profile pictures and social media"
                }
            }
            
            // 21:9 Ultrawide
            div { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-3",
                    "21:9 Ultrawide Format"
                }
                div { class: "max-w-2xl",
                    AspectRatio { 
                        ratio: 21.0 / 9.0,
                        class: "bg-muted rounded-lg overflow-hidden",
                        img {
                            class: "h-full w-full object-cover",
                            src: "https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=1200&dpr=2&q=80",
                            alt: "Wide landscape view",
                        }
                    }
                }
                p { class: "text-sm text-muted-foreground mt-2",
                    "Cinematic ultrawide format"
                }
            }
            
            // Custom Content Example
            div { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-3",
                    "Custom Content"
                }
                div { class: "max-w-md",
                    AspectRatio { 
                        ratio: 3.0 / 2.0,
                        class: "bg-secondary/50 rounded-lg border-2 border-dashed border-border",
                        div { class: "h-full w-full flex flex-col items-center justify-center p-6 text-center",
                            div { class: "w-16 h-16 bg-primary/10 rounded-full flex items-center justify-center mb-4",
                                span { class: "text-2xl", "🎨" }
                            }
                            h4 { class: "text-lg font-semibold text-foreground mb-2", "Custom Content" }
                            p { class: "text-sm text-muted-foreground", 
                                "You can put any content inside an AspectRatio container"
                            }
                            button { 
                                class: "mt-4 px-4 py-2 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:bg-primary/90 transition-colors",
                                "Click me"
                            }
                        }
                    }
                }
                p { class: "text-sm text-muted-foreground mt-2",
                    "Any content can be placed inside"
                }
            }
            
            // Multiple Aspect Ratios Grid
            div { class: "mb-8",
                h3 { class: "text-lg font-semibold mb-3",
                    "Multiple Ratios Grid"
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    // 9:16 Portrait
                    div {
                        AspectRatio { 
                            ratio: 9.0 / 16.0,
                            class: "bg-gradient-to-b from-blue-500/20 to-purple-500/20 rounded-lg",
                            div { class: "h-full w-full flex items-center justify-center",
                                div { class: "text-center text-white",
                                    h5 { class: "font-bold text-foreground", "9:16" }
                                    p { class: "text-sm text-muted-foreground", "Portrait" }
                                }
                            }
                        }
                        p { class: "text-xs text-muted-foreground mt-1 text-center", "Mobile/Stories" }
                    }
                    
                    // 1:1 Square
                    div {
                        AspectRatio { 
                            ratio: 1.0,
                            class: "bg-gradient-to-br from-green-500/20 to-emerald-500/20 rounded-lg",
                            div { class: "h-full w-full flex items-center justify-center",
                                div { class: "text-center",
                                    h5 { class: "font-bold text-foreground", "1:1" }
                                    p { class: "text-sm text-muted-foreground", "Square" }
                                }
                            }
                        }
                        p { class: "text-xs text-muted-foreground mt-1 text-center", "Instagram" }
                    }
                    
                    // 16:9 Landscape
                    div {
                        AspectRatio { 
                            ratio: 16.0 / 9.0,
                            class: "bg-gradient-to-r from-orange-500/20 to-red-500/20 rounded-lg",
                            div { class: "h-full w-full flex items-center justify-center",
                                div { class: "text-center",
                                    h5 { class: "font-bold text-foreground", "16:9" }
                                    p { class: "text-sm text-muted-foreground", "Landscape" }
                                }
                            }
                        }
                        p { class: "text-xs text-muted-foreground mt-1 text-center", "YouTube" }
                    }
                }
            }
        }
    }
}
