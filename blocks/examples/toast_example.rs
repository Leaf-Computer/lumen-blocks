use dioxus::prelude::*;
use dioxus_blocks::components::button::{Button, ButtonVariant};
use dioxus_blocks::components::toast::{use_toast, ToastOptions, ToastProvider, ToastType};
use std::time::Duration;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ToastProvider {
            default_duration: Duration::from_secs(4),
            max_toasts: 5,
            
            div {
                class: "min-h-screen bg-background p-8",
                
                div {
                    class: "max-w-4xl mx-auto space-y-8",
                    
                    h1 {
                        class: "text-3xl font-bold text-foreground mb-8",
                        "Toast Notification Examples"
                    }
                    
                    ToastExamples {}
                }
            }
        }
    }
}

#[component]
pub fn ToastExamples() -> Element {
    let toasts = use_toast();

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-6",
            
            // Basic Toast Types
            div {
                class: "space-y-4",
                
                h2 {
                    class: "text-xl font-semibold text-foreground mb-4",
                    "Basic Toast Types"
                }
                
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
            
            // Toast with Descriptions
            div {
                class: "space-y-4",
                
                h2 {
                    class: "text-xl font-semibold text-foreground mb-4",
                    "Toasts with Descriptions"
                }
                
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
            
            // Custom Duration Toasts
            div {
                class: "space-y-4",
                
                h2 {
                    class: "text-xl font-semibold text-foreground mb-4",
                    "Custom Duration"
                }
                
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
            
            // Advanced Examples
            div {
                class: "space-y-4",
                
                h2 {
                    class: "text-xl font-semibold text-foreground mb-4",
                    "Advanced Examples"
                }
                
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
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
                        
                        // Show success after a delay (in a real app, this would be triggered by upload completion)
                        toasts.success(
                            "Upload complete".to_string(),
                            Some(ToastOptions {
                                description: Some("Your file has been successfully uploaded.".to_string()),
                                duration: Some(Duration::from_secs(3)),
                                ..Default::default()
                            })
                        );
                    },
                    "Simulate Upload Process"
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
            }
        }
        
        div {
            class: "mt-8 p-4 bg-muted rounded-lg",
            h3 {
                class: "text-lg font-semibold mb-2",
                "Usage Tips"
            }
            ul {
                class: "space-y-1 text-sm text-muted-foreground",
                li { "• Success toasts are great for confirming actions" }
                li { "• Error toasts should provide actionable information" }
                li { "• Warning toasts help prevent user mistakes" }
                li { "• Info toasts can announce new features or tips" }
                li { "• Use permanent toasts for critical errors that need attention" }
                li { "• Keep toast messages concise and clear" }
            }
        }
    }
}