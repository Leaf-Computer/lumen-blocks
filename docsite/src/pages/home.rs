use dioxus::prelude::*;
use laminar_blocks::components::{
    avatar::{Avatar, AvatarImage, AvatarFallback},
    button::{Button, ButtonVariant, ButtonSize},
    input::{Input, InputSize},
    progress::{Progress, ProgressVariant},
    switch::Switch,
};
use lucide_dioxus::{Check, Info, X};
use docs::docs;

use crate::Route;
use crate::LAMINAR_LOGO;
use crate::components::FeatureCard;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-background relative",
            
            div { class: "max-w-6xl mx-auto px-6 py-12",
                div { class: "text-center mb-12",
                    img { class: "w-48 h-48 mx-auto mb-4", src: LAMINAR_LOGO, alt: "Laminar Logo" }
                    h1 { class: "text-4xl font-bold text-foreground mb-4", "Laminar Blocks Demo" }
                    p { class: "text-xl text-muted-foreground mb-8", 
                        "A comprehensive component library for Dioxus applications" 
                    }
                    
                    div { class: "flex justify-center gap-4",
                        Link { to: Route::Docs01 { child: docs::router_01::BookRoute::Index { section: Default::default() } },
                            Button {
                                variant: use_signal(|| ButtonVariant::Primary),
                                size: use_signal(|| ButtonSize::Large),
                                "View Docs"
                            }
                        }
                    }
                }
                
                // Feature Cards
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 mb-12",
                    FeatureCard {
                        title: "Rich Components".to_string(),
                        description: "Comprehensive set of UI components built for modern web applications".to_string(),
                        icon: rsx! { Check { class: "w-8 h-8 text-primary" } }
                    }
                    FeatureCard {
                        title: "Tailwind Styled".to_string(),
                        description: "Beautifully designed with Tailwind CSS and dark mode support".to_string(),
                        icon: rsx! { Info { class: "w-8 h-8 text-primary" } }
                    }
                    FeatureCard {
                        title: "Accessible".to_string(),
                        description: "Built with accessibility in mind, following ARIA best practices".to_string(),
                        icon: rsx! { X { class: "w-8 h-8 text-primary" } }
                    }
                }
                
                // Quick Component Preview
                div { class: "bg-card rounded-lg border border-border p-8",
                    h2 { class: "text-2xl font-semibold text-foreground mb-6", "Quick Preview" }
                    
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                        div { class: "space-y-4",
                            h3 { class: "font-medium text-foreground mb-3", "Buttons" }
                            div { class: "flex flex-wrap gap-2",
                                Button {
                                    variant: use_signal(|| ButtonVariant::Primary),
                                    "Primary"
                                }
                                Button {
                                    variant: use_signal(|| ButtonVariant::Secondary),
                                    "Secondary"
                                }
                                Button {
                                    variant: use_signal(|| ButtonVariant::Outline),
                                    "Outline"
                                }
                            }
                            
                            h3 { class: "font-medium text-foreground mb-3 mt-6", "Inputs" }
                            Input {
                                placeholder: use_signal(|| "Enter your email".to_string()),
                                size: use_signal(|| InputSize::Medium),
                                icon_left: rsx! { Info { class: "w-4 h-4" } },
                            }
                            
                            h3 { class: "font-medium text-foreground mb-3 mt-6", "Progress" }
                            Progress {
                                value: Some(use_signal(|| 75.0).into()),
                                variant: ProgressVariant::Default,
                                show_percentage: true,
                            }
                        }
                        
                        div { class: "space-y-4",
                            h3 { class: "font-medium text-foreground mb-3", "Avatars" }
                            div { class: "flex gap-2",
                                Avatar {
                                    AvatarImage {
                                        src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=40&h=40&fit=crop&crop=face".to_string(),
                                        alt: "User".to_string(),
                                    }
                                    AvatarFallback { "JD" }
                                }
                                Avatar {
                                    AvatarFallback { "AB" }
                                }
                                Avatar {
                                    class: Some("w-12 h-12".to_string()),
                                    AvatarFallback { "CD" }
                                }
                            }
                            
                            h3 { class: "font-medium text-foreground mb-3 mt-6", "Switch" }
                            div { class: "flex items-center gap-3",
                                Switch {
                                    checked: use_signal(|| true),
                                }
                                span { class: "text-sm text-muted-foreground", "Enable notifications" }
                            }
                        }
                    }
                }
            }
        }
    }
}
