use dioxus::prelude::*;
use laminar_blocks::components::{
    avatar::{Avatar, AvatarImage, AvatarFallback},
    button::{Button, ButtonVariant, ButtonSize},
    input::{Input, InputSize, InputVariant},
    progress::{Progress, ProgressSize, ProgressVariant},
    switch::{Switch, SwitchSize},
};
use lucide_dioxus::{Check, Info, X};

#[component]
pub fn ComponentShowcase(
    mut notification_enabled: Signal<bool>,
    mut dark_mode: Signal<bool>
) -> Element {
    rsx! {
        div { class: "space-y-8",
            div {
                h3 { class: "text-lg font-semibold text-foreground mb-2", "Component Showcase" }
                p { class: "text-muted-foreground", "Interactive examples of the blocks component library" }
            }
            
            // Button Showcase
            div { class: "bg-card rounded-lg border border-border p-6",
                h4 { class: "text-md font-semibold text-foreground mb-4", "Buttons" }
                div { class: "space-y-4",
                    div {
                        h5 { class: "text-sm font-medium text-foreground mb-3", "Variants" }
                        div { class: "flex flex-wrap gap-3",
                            Button {
                                variant: Signal::new(ButtonVariant::Primary),
                                "Primary"
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Secondary),
                                "Secondary"
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Outline),
                                "Outline"
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Ghost),
                                "Ghost"
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Link),
                                "Link"
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Destructive),
                                "Destructive"
                            }
                        }
                    }
                    
                    div {
                        h5 { class: "text-sm font-medium text-foreground mb-3", "Sizes" }
                        div { class: "flex flex-wrap gap-3",
                            Button {
                                variant: Signal::new(ButtonVariant::Primary),
                                size: Signal::new(ButtonSize::Small),
                                "Small"
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Primary),
                                size: Signal::new(ButtonSize::Medium),
                                "Medium"
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Primary),
                                size: Signal::new(ButtonSize::Large),
                                "Large"
                            }
                        }
                    }
                    
                    div {
                        h5 { class: "text-sm font-medium text-foreground mb-3", "With Icons & States" }
                        div { class: "flex flex-wrap gap-3",
                            Button {
                                variant: Signal::new(ButtonVariant::Primary),
                                icon_left: rsx! { Check { class: "w-4 h-4" } },
                                "With Icon"
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Outline),
                                loading: Signal::new(true),
                                "Loading"
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Primary),
                                is_icon_button: Signal::new(true),
                                aria_label: Some("Settings".to_string()),
                                Info { class: "w-4 h-4" }
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Secondary),
                                disabled: Signal::new(true),
                                "Disabled"
                            }
                        }
                    }
                }
            }
            
            // Input Showcase
            div { class: "bg-card rounded-lg border border-border p-6",
                h4 { class: "text-md font-semibold text-foreground mb-4", "Inputs" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div { class: "space-y-4",
                        h5 { class: "text-sm font-medium text-foreground mb-3", "Basic Inputs" }
                        Input {
                            placeholder: Signal::new("Default input".to_string()),
                            size: Signal::new(InputSize::Medium),
                        }
                        Input {
                            placeholder: Signal::new("With left icon".to_string()),
                            size: Signal::new(InputSize::Medium),
                            icon_left: rsx! { Check { class: "w-4 h-4" } },
                        }
                        Input {
                            placeholder: Signal::new("With right icon".to_string()),
                            size: Signal::new(InputSize::Medium),
                            icon_right: rsx! { Info { class: "w-4 h-4" } },
                        }
                    }
                    div { class: "space-y-4",
                        h5 { class: "text-sm font-medium text-foreground mb-3", "Sizes & States" }
                        Input {
                            placeholder: Signal::new("Small input".to_string()),
                            size: Signal::new(InputSize::Small),
                        }
                        Input {
                            placeholder: Signal::new("Large input".to_string()),
                            size: Signal::new(InputSize::Large),
                        }
                        Input {
                            placeholder: Signal::new("Error state".to_string())(),
                            size: Signal::new(InputSize::Medium),
                            variant: Signal::new(InputVariant::Error),
                        }
                    }
                }
            }
            
            // Switch Showcase
            div { class: "bg-card rounded-lg border border-border p-6",
                h4 { class: "text-md font-semibold text-foreground mb-4", "Switches" }
                div { class: "space-y-6",
                    div { class: "space-y-4",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "font-medium text-foreground", "Enable Notifications" }
                                p { class: "text-sm text-muted-foreground", "Receive push notifications for updates" }
                            }
                            Switch {
                                checked: notification_enabled,
                                on_checked_change: Some(EventHandler::new(move |checked| {
                                    notification_enabled.set(checked);
                                })),
                                size: SwitchSize::Medium,
                            }
                        }
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "font-medium text-foreground", "Dark Mode" }
                                p { class: "text-sm text-muted-foreground", "Switch to dark theme" }
                            }
                            Switch {
                                checked: dark_mode,
                                on_checked_change: Some(EventHandler::new(move |checked| {
                                    dark_mode.set(checked);
                                })),
                                size: SwitchSize::Medium,
                            }
                        }
                    }
                    
                    div {
                        h5 { class: "text-sm font-medium text-foreground mb-3", "Sizes" }
                        div { class: "flex items-center gap-6",
                            div { class: "flex items-center gap-2",
                                span { class: "text-xs text-muted-foreground min-w-[20px]", "Small" }
                                Switch {
                                    checked: Signal::new(true),
                                    size: SwitchSize::Small,
                                }
                            }
                            div { class: "flex items-center gap-2",
                                span { class: "text-xs text-muted-foreground min-w-[20px]", "Medium" }
                                Switch {
                                    checked: Signal::new(true),
                                    size: SwitchSize::Medium,
                                }
                            }
                            div { class: "flex items-center gap-2",
                                span { class: "text-xs text-muted-foreground min-w-[20px]", "Large" }
                                Switch {
                                    checked: Signal::new(true),
                                    size: SwitchSize::Large,
                                }
                            }
                        }
                    }
                }
            }
            
            // Progress Showcase
            div { class: "bg-card rounded-lg border border-border p-6",
                h4 { class: "text-md font-semibold text-foreground mb-4", "Progress Indicators" }
                div { class: "space-y-6",
                    div { class: "space-y-4",
                        h5 { class: "text-sm font-medium text-foreground mb-3", "Variants" }
                        Progress {
                            value: Some(Signal::new(65.0).into()),
                            variant: ProgressVariant::Default,
                            show_percentage: true,
                            aria_label: Some("Default Progress".to_string()),
                        }
                        Progress {
                            value: Some(Signal::new(85.0).into()),
                            variant: ProgressVariant::Success,
                            show_percentage: true,
                            aria_label: Some("Success Progress".to_string()),
                        }
                        Progress {
                            value: Some(Signal::new(45.0).into()),
                            variant: ProgressVariant::Warning,
                            show_percentage: true,
                            aria_label: Some("Warning Progress".to_string()),
                        }
                        Progress {
                            value: Some(Signal::new(25.0).into()),
                            variant: ProgressVariant::Destructive,
                            show_percentage: true,
                            aria_label: Some("Destructive Progress".to_string()),
                        }
                    }
                    
                    div { class: "space-y-4",
                        h5 { class: "text-sm font-medium text-foreground mb-3", "Sizes" }
                        Progress {
                            value: Some(Signal::new(70.0).into()),
                            size: ProgressSize::Small,
                            show_percentage: true,
                            aria_label: Some("Small Progress".to_string()),
                        }
                        Progress {
                            value: Some(Signal::new(70.0).into()),
                            size: ProgressSize::Medium,
                            show_percentage: true,
                            aria_label: Some("Medium Progress".to_string()),
                        }
                        Progress {
                            value: Some(Signal::new(70.0).into()),
                            size: ProgressSize::Large,
                            show_percentage: true,
                            aria_label: Some("Large Progress".to_string()),
                        }
                    }
                }
            }
            
            // Avatar Showcase
            div { class: "bg-card rounded-lg border border-border p-6",
                h4 { class: "text-md font-semibold text-foreground mb-4", "Avatars" }
                div { class: "space-y-4",
                    div {
                        h5 { class: "text-sm font-medium text-foreground mb-3", "Sizes & Types" }
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
                    }
                    div {
                        h5 { class: "text-sm font-medium text-foreground mb-3", "Avatar Group" }
                        div { class: "flex -space-x-2",
                            Avatar {
                                class: Some("w-10 h-10 border-2 border-background".to_string()),
                                AvatarImage {
                                    src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=40&h=40&fit=crop&crop=face".to_string(),
                                    alt: "User 1".to_string(),
                                }
                                AvatarFallback { "U1" }
                            }
                            Avatar {
                                class: Some("w-10 h-10 border-2 border-background".to_string()),
                                AvatarImage {
                                    src: "https://images.unsplash.com/photo-1494790108755-2616b612b786?w=40&h=40&fit=crop&crop=face".to_string(),
                                    alt: "User 2".to_string(),
                                }
                                AvatarFallback { "U2" }
                            }
                            Avatar {
                                class: Some("w-10 h-10 border-2 border-background".to_string()),
                                AvatarFallback { "U3" }
                            }
                            div { class: "w-10 h-10 bg-muted border-2 border-background rounded-full flex items-center justify-center",
                                span { class: "text-xs text-muted-foreground", "+5" }
                            }
                        }
                    }
                    p { class: "text-sm text-muted-foreground", "Avatars with images and fallback text, plus group layouts" }
                }
            }
        }
    }
}
