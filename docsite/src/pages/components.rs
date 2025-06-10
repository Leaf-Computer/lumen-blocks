use dioxus::prelude::*;
use laminar_blocks::components::{
    button::{Button, ButtonSize, ButtonVariant},
    input::{Input, InputSize, InputVariant},
    switch::Switch,
    avatar::{Avatar, AvatarImage, AvatarFallback},
    progress::{Progress, ProgressSize, ProgressVariant},
};
use lucide_dioxus::{Check, Info};

use crate::components::{
    ToastSection,
    ProgressSection,
    FormSection,
};

#[component]
pub fn Components() -> Element {
    let notification_enabled = use_signal(|| true);
    let dark_mode = use_signal(|| false);
    
    rsx! {
        div { class: "min-h-screen bg-background",
            div { class: "bg-card shadow-sm border-b border-border",
                div { class: "max-w-6xl mx-auto px-6 py-4 flex items-center justify-between",
                    h1 { class: "text-xl font-bold", "Dioxus Blocks" }
                    
                    div { class: "flex items-center gap-4",
                        a { 
                            href: "/", 
                            class: "text-sm font-medium text-muted-foreground hover:text-foreground transition-colors",
                            "Home"
                        }
                        a { 
                            href: "/components", 
                            class: "text-sm font-medium text-foreground",
                            "Components"
                        }
                        a { 
                            href: "/dashboard", 
                            class: "text-sm font-medium text-muted-foreground hover:text-foreground transition-colors",
                            "Dashboard"
                        }
                    }
                }
            }
            
            div { class: "max-w-6xl mx-auto px-6 py-8",
                h1 { class: "text-3xl font-bold text-foreground mb-2", "Component Showcase" }
                p { class: "text-muted-foreground mb-8", 
                    "A comprehensive demonstration of the Dioxus Blocks component library" 
                }
                
                div { class: "space-y-12",
                    // Toast Section - New Component
                    ToastSection {}
                    
                    // Progress Section - New Component
                    ProgressSection {}
                    
                    // Form Elements Section - New Component
                    FormSection {}
                    
                    // Buttons Section
                    ButtonsSection {}
                }
            }
        }
    }
}

#[component]
fn ButtonsSection() -> Element {
    rsx! {
        section { class: "mb-10",
            h2 { class: "text-2xl font-bold mb-6", "Buttons" }
            div { class: "p-6 bg-card rounded-lg shadow-sm border border-border",
                p { class: "text-muted-foreground mb-6", 
                    "Interactive buttons with multiple variants and sizes"
                }
                div { class: "space-y-6",
                div { class: "space-y-6",
                    div {
                                h4 { class: "font-medium text-foreground mb-3", "Variants" }
                                div { class: "flex flex-wrap gap-3",
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
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Ghost),
                                        "Ghost"
                                    }
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Link),
                                        "Link"
                                    }
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Destructive),
                                        "Destructive"
                                    }
                                }
                            }
                            
                            div {
                                h4 { class: "font-medium text-foreground mb-3", "Sizes" }
                                div { class: "flex flex-wrap gap-3",
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Primary),
                                        size: use_signal(|| ButtonSize::Small),
                                        "Small"
                                    }
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Primary),
                                        size: use_signal(|| ButtonSize::Medium),
                                        "Medium"
                                    }
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Primary),
                                        size: use_signal(|| ButtonSize::Large),
                                        "Large"
                                    }
                                }
                            }
                            
                            div {
                                h4 { class: "font-medium text-foreground mb-3", "With Icons & States" }
                                div { class: "flex flex-wrap gap-3",
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Primary),
                                        icon_left: rsx! { Check { class: "w-4 h-4" } },
                                        "With Icon"
                                    }
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Outline),
                                        loading: use_signal(|| true),
                                        "Loading"
                                    }
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Primary),
                                        is_icon_button: use_signal(|| true),
                                        aria_label: Some("Settings".to_string()),
                                        Info { class: "w-4 h-4" }
                                    }
                                    Button {
                                        variant: use_signal(|| ButtonVariant::Primary),
                                        disabled: use_signal(|| true),
                                        "Disabled"
                                    }
                                }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InputsSection() -> Element {
    rsx! {
        section { class: "mb-10",
            h2 { class: "text-2xl font-bold mb-6", "Inputs" }
            div { class: "p-6 bg-card rounded-lg shadow-sm border border-border",
                p { class: "text-muted-foreground mb-6", 
                    "Form inputs with validation states and icons"
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div { class: "space-y-4",
                        Input {
                            placeholder: use_signal(|| "Default input".to_string()),
                            size: use_signal(|| InputSize::Medium),
                        }
                        Input {
                            placeholder: use_signal(|| "With left icon".to_string()),
                            size: use_signal(|| InputSize::Medium),
                            icon_left: rsx! { Check { class: "w-4 h-4" } },
                        }
                        Input {
                            placeholder: use_signal(|| "With right icon".to_string()),
                            size: use_signal(|| InputSize::Medium),
                            icon_right: rsx! { Info { class: "w-4 h-4" } },
                        }
                    }
                    div { class: "space-y-4",
                                Input {
                                    placeholder: use_signal(|| "Small input".to_string()),
                                    size: use_signal(|| InputSize::Small),
                                }
                                Input {
                                    placeholder: use_signal(|| "Large input".to_string()),
                                    size: use_signal(|| InputSize::Large),
                                }
                                Input {
                                    placeholder: use_signal(|| "Error state".to_string()),
                                    size: use_signal(|| InputSize::Medium),
                                    variant: use_signal(|| InputVariant::Error),
                                }
                    }
                }
            }
        }
    }
}

#[component]
fn ProgressBarsSection() -> Element {
    rsx! {
        section { class: "mb-10",
            h2 { class: "text-2xl font-bold mb-6", "Progress Bars" }
            div { class: "p-6 bg-card rounded-lg shadow-sm border border-border",
                p { class: "text-muted-foreground mb-6", 
                    "Progress indicators with different variants and sizes"
                }
                div { class: "space-y-6",
                    div { class: "space-y-3",
                        h4 { class: "font-medium text-foreground mb-3", "Variants" }
                        Progress {
                            value: Some(use_signal(|| 65.0).into()),
                            variant: ProgressVariant::Default,
                            show_percentage: true,
                        }
                                Progress {
                                    value: Some(use_signal(|| 85.0).into()),
                                    variant: ProgressVariant::Success,
                                    show_percentage: true,
                                }
                                Progress {
                                    value: Some(use_signal(|| 45.0).into()),
                                    variant: ProgressVariant::Warning,
                                    show_percentage: true,
                                }
                                Progress {
                                    value: Some(use_signal(|| 25.0).into()),
                                    variant: ProgressVariant::Destructive,
                                    show_percentage: true,
                                }
                    }
                    
                    div { class: "space-y-3",
                        h4 { class: "font-medium text-foreground mb-3", "Sizes" }
                        Progress {
                            value: Some(use_signal(|| 70.0).into()),
                            size: ProgressSize::Small,
                            show_percentage: true,
                        }
                                Progress {
                                    value: Some(use_signal(|| 70.0).into()),
                                    size: ProgressSize::Medium,
                                    show_percentage: true,
                                }
                                Progress {
                                    value: Some(use_signal(|| 70.0).into()),
                                    size: ProgressSize::Large,
                                    show_percentage: true,
                                }
                    }
                }
            }
        }
    }
}

#[component]
fn SwitchesSection() -> Element {
    let mut notification_enabled = use_signal(|| true);
    
    rsx! {
        section { class: "mb-10",
            h2 { class: "text-2xl font-bold mb-6", "Switches" }
            div { class: "p-6 bg-card rounded-lg shadow-sm border border-border",
                p { class: "text-muted-foreground mb-6", 
                    "Toggle switches for settings and preferences"
                }
                div { class: "space-y-4",
                    div { class: "flex items-center justify-between p-4 border border-border rounded-lg",
                        div {
                            p { class: "font-medium text-foreground", "Enable Notifications" }
                            p { class: "text-sm text-muted-foreground", "Receive push notifications for updates" }
                        }
                        Switch {
                            checked: notification_enabled,
                            on_checked_change: move |checked| {
                                notification_enabled.set(checked);
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AvatarsSection() -> Element {
    rsx! {
        section { class: "mb-10",
            h2 { class: "text-2xl font-bold mb-6", "Avatars" }
            div { class: "p-6 bg-card rounded-lg shadow-sm border border-border",
                p { class: "text-muted-foreground mb-6", 
                    "User avatars with images and fallbacks"
                }
                div { class: "space-y-4",
                    div { class: "flex items-center gap-4",
                        Avatar {
                            AvatarImage {
                                src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=40&h=40&fit=crop&crop=face".to_string(),
                                alt: "Avatar".to_string(),
                            }
                            AvatarFallback { "JD" }
                        }
                                Avatar {
                                    class: Some("w-12 h-12".to_string()),
                                    AvatarImage {
                                        src: "https://images.unsplash.com/photo-1494790108755-2616b612b786?w=48&h=48&fit=crop&crop=face".to_string(),
                                        alt: "Avatar".to_string(),
                                    }
                                    AvatarFallback { "AJ" }
                                }
                                Avatar {
                                    class: Some("w-16 h-16".to_string()),
                                    AvatarFallback { "CS" }
                                }
                            }
                    p { class: "text-sm text-muted-foreground", "Avatars with images and fallback text" }
                }
            }
        }
    }
}
