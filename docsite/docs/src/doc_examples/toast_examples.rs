#![allow(non_snake_case)]

pub use setup::ToastProviderSetupExample;
pub use basic::BasicToastExample;
pub use descriptions::ToastWithDescriptionsExample;
pub use durations::CustomDurationToastExample;
pub use advanced::AdvancedToastExample;

pub mod setup {
    // ANCHOR: setup
    use dioxus::prelude::*;
    use laminar_blocks::components::toast::{ToastProvider};
    use std::time::Duration;
    
    #[component]
    pub fn ToastProviderSetupExample() -> Element {
        rsx! {
            // Wrap your application or a component with ToastProvider
            ToastProvider {
                // Default duration for toasts (optional)
                default_duration: Duration::from_secs(4),
                // Maximum number of toasts to show at once (optional)
                max_toasts: 5,
                
                // Your application content goes here
                div {
                    class: "p-4 border rounded-md bg-muted",
                    p {
                        class: "text-center text-muted-foreground",
                        "ToastProvider is configured and ready to use!"
                    }
                    p {
                        class: "text-center text-xs text-muted-foreground mt-2",
                        "Note: This example only shows the setup. See other examples for actual toast usage."
                    }
                }
            }
        }
    }
    // ANCHOR_END: setup
}

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use laminar_blocks::components::toast::use_toast;
    
    #[component]
    pub fn BasicToastExample() -> Element {
        let toasts = use_toast();
        
        rsx! {
            div { class: "space-y-4",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.success("Success!".to_string(), None);
                    },
                    "Show Success Toast"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Destructive),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.error("Error occurred!".to_string(), None);
                    },
                    "Show Error Toast"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Secondary),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.warning("Warning message".to_string(), None);
                    },
                    "Show Warning Toast"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Outline),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.info("Information".to_string(), None);
                    },
                    "Show Info Toast"
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod descriptions {
    // ANCHOR: descriptions
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use laminar_blocks::components::toast::{use_toast, ToastOptions};
    
    #[component]
    pub fn ToastWithDescriptionsExample() -> Element {
        let toasts = use_toast();
        
        rsx! {
            div { class: "space-y-4",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.success(
                            "Profile updated".to_string(),
                            Some(ToastOptions {
                                description: Some("Your profile has been successfully updated.".to_string()),
                                ..Default::default()
                            })
                        );
                    },
                    "Success with Description"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Destructive),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.error(
                            "Network Error".to_string(),
                            Some(ToastOptions {
                                description: Some("Failed to connect to the server. Please try again.".to_string()),
                                ..Default::default()
                            })
                        );
                    },
                    "Error with Description"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Secondary),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.warning(
                            "Storage Almost Full".to_string(),
                            Some(ToastOptions {
                                description: Some("You have used 90% of your storage quota.".to_string()),
                                ..Default::default()
                            })
                        );
                    },
                    "Warning with Description"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Outline),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.info(
                            "New Feature Available".to_string(),
                            Some(ToastOptions {
                                description: Some("Check out the new dashboard in your settings.".to_string()),
                                ..Default::default()
                            })
                        );
                    },
                    "Info with Description"
                }
            }
        }
    }
    // ANCHOR_END: descriptions
}

pub mod durations {
    // ANCHOR: durations
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use laminar_blocks::components::toast::{use_toast, ToastOptions};
    use std::time::Duration;
    
    #[component]
    pub fn CustomDurationToastExample() -> Element {
        let toasts = use_toast();
        
        rsx! {
            div { class: "space-y-4",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.success(
                            "Quick toast".to_string(),
                            Some(ToastOptions {
                                description: Some("This toast disappears in 1 second".to_string()),
                                duration: Some(Duration::from_secs(1)),
                                ..Default::default()
                            })
                        );
                    },
                    "1 Second Toast"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Secondary),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.info(
                            "Long toast".to_string(),
                            Some(ToastOptions {
                                description: Some("This toast stays for 10 seconds".to_string()),
                                duration: Some(Duration::from_secs(10)),
                                ..Default::default()
                            })
                        );
                    },
                    "10 Second Toast"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Destructive),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        toasts.error(
                            "Permanent Error".to_string(),
                            Some(ToastOptions {
                                description: Some("This toast stays until manually closed".to_string()),
                                permanent: true,
                                ..Default::default()
                            })
                        );
                    },
                    "Permanent Toast"
                }
            }
        }
    }
    // ANCHOR_END: durations
}

pub mod advanced {
    // ANCHOR: advanced
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use laminar_blocks::components::toast::{use_toast, ToastOptions, ToastType};
    use std::time::Duration;
    
    #[component]
    pub fn AdvancedToastExample() -> Element {
        let toasts = use_toast();
        
        rsx! {
            div { class: "space-y-4",
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        // Custom toast using the show method directly
                        toasts.show(
                            "Custom Toast".to_string(),
                            ToastType::Success,
                            ToastOptions {
                                description: Some("This toast was created using the show() method directly.".to_string()),
                                duration: Some(Duration::from_secs(6)),
                                permanent: false,
                            }
                        );
                    },
                    "Custom Show Method"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Secondary),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        // Show multiple toasts of different types
                        toasts.info("Processing...".to_string(), None);
                        toasts.warning(
                            "Rate limit warning".to_string(),
                            Some(ToastOptions {
                                description: Some("You're approaching your API rate limit.".to_string()),
                                ..Default::default()
                            })
                        );
                        toasts.success(
                            "Task completed".to_string(),
                            Some(ToastOptions {
                                description: Some("All operations finished successfully.".to_string()),
                                ..Default::default()
                            })
                        );
                    },
                    "Show Multiple Toasts"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Outline),
                    full_width: use_signal(|| true),
                    on_click: move |_| {
                        // Simulate a file upload process
                        toasts.info(
                            "Upload started".to_string(),
                            Some(ToastOptions {
                                description: Some("Your file is being uploaded...".to_string()),
                                duration: Some(Duration::from_millis(2000)),
                                ..Default::default()
                            })
                        );
                        
                        // Show success after a delay
                        toasts.success(
                            "Upload complete".to_string(),
                            Some(ToastOptions {
                                description: Some("Your file has been successfully uploaded.".to_string()),
                                duration: Some(Duration::from_secs(3)),
                                ..Default::default()
                            })
                        );
                    },
                    "Simulate Process Flow"
                }
            }
        }
    }
    // ANCHOR_END: advanced
}

// This is for backward compatibility with any existing examples
pub mod example {
    use dioxus::prelude::*;
    use laminar_blocks::components::{button::Button, toast::use_toast};
    
    #[component]
    pub fn ToastExample() -> Element {
        let toasts = use_toast();
        
        rsx! {
            div {
                Button {
                    on_click: move |_| {
                        toasts.success("Success!".to_string(), None);
                    },
                    "Show Toast"
                }
            }
        }
    }
}
