use dioxus_lib::prelude::*;
use dioxus_blocks::components::toast::{
    ToastProvider, use_toast
};
use dioxus_primitives::toast::ToastOptions;
use std::time::Duration;

fn main() {
    #[cfg(debug_assertions)]
    {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        log::info!("Logger initialized in debug mode");
    }

    dioxus::launch(ToastStandaloneExample);
}

#[component]
pub fn ToastStandaloneExample() -> Element {
    rsx! {
        ToastProvider {
            ToastExample {}
        }
    }
}

#[component]
pub fn ToastExample() -> Element {
    rsx! {
        div { class: "toast-example p-12 bg-background min-h-screen",
            div { class: "max-w-4xl mx-auto",
                h2 { class: "mb-8 text-3xl font-bold text-foreground", "Toast Notifications Example" }
                p { class: "mb-8 text-muted-foreground text-lg", 
                    "Click the buttons below to see different types of toast notifications in action."
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                    // Basic Toast Types
                    div { class: "space-y-6",
                        h3 { class: "text-xl font-semibold mb-4", "Basic Toast Types" }
                        
                        div { class: "space-y-3",
                            BasicToastButtons {}
                        }
                    }

                    // Advanced Toast Features
                    div { class: "space-y-6",
                        h3 { class: "text-xl font-semibold mb-4", "Advanced Features" }
                        
                        div { class: "space-y-3",
                            AdvancedToastButtons {}
                        }
                    }
                }

                div { class: "mt-12 p-6 rounded-lg bg-muted/50 border",
                    h4 { class: "text-lg font-semibold mb-3", "Toast Features" }
                    ul { class: "space-y-2 text-sm text-muted-foreground",
                        li { "• Auto-dismiss with customizable duration" }
                        li { "• Manual close option for important messages" }
                        li { "• Different variants: Success, Error, Warning, Info" }
                        li { "• Rich content with titles and descriptions" }
                        li { "• Action buttons for interactive toasts" }
                        li { "• Responsive design and dark mode support" }
                        li { "• Accessible with proper ARIA labels" }
                        li { "• Smooth animations and transitions" }
                    }
                }
            }
        }
    }
}

#[component]
fn BasicToastButtons() -> Element {
    let toast_api = use_toast();

    rsx! {
        div { class: "grid grid-cols-2 gap-3",
            button {
                class: "px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors",
                onclick: move |_| {
                    toast_api.success(
                        "Success!".to_string(),
                        Some(ToastOptions {
                            description: Some("Your action was completed successfully.".to_string()),
                            duration: Some(Duration::from_secs(3)),
                            ..Default::default()
                        }),
                    );
                },
                "Success Toast"
            }

            button {
                class: "px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 transition-colors",
                onclick: move |_| {
                    toast_api.error(
                        "Error occurred".to_string(),
                        Some(ToastOptions {
                            description: Some("Something went wrong. Please try again.".to_string()),
                            duration: Some(Duration::from_secs(5)),
                            ..Default::default()
                        }),
                    );
                },
                "Error Toast"
            }

            button {
                class: "px-4 py-2 bg-yellow-600 text-white rounded-md hover:bg-yellow-700 transition-colors",
                onclick: move |_| {
                    toast_api.warning(
                        "Warning".to_string(),
                        Some(ToastOptions {
                            description: Some("This action may have consequences.".to_string()),
                            duration: Some(Duration::from_secs(4)),
                            ..Default::default()
                        }),
                    );
                },
                "Warning Toast"
            }

            button {
                class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors",
                onclick: move |_| {
                    toast_api.info(
                        "Information".to_string(),
                        Some(ToastOptions {
                            description: Some("Here's some helpful information for you.".to_string()),
                            duration: Some(Duration::from_secs(3)),
                            ..Default::default()
                        }),
                    );
                },
                "Info Toast"
            }
        }
    }
}

#[component]
fn AdvancedToastButtons() -> Element {
    let toast_api = use_toast();

    rsx! {
        div { class: "space-y-3",
            button {
                class: "w-full px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition-colors",
                onclick: move |_| {
                    toast_api.success(
                        "Permanent Success".to_string(),
                        Some(ToastOptions {
                            description: Some("This toast won't disappear automatically.".to_string()),
                            permanent: true,
                            ..Default::default()
                        }),
                    );
                },
                "Permanent Toast"
            }

            button {
                class: "w-full px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 transition-colors",
                onclick: move |_| {
                    toast_api.info(
                        "Quick Message".to_string(),
                        Some(ToastOptions {
                            duration: Some(Duration::from_secs(1)),
                            ..Default::default()
                        }),
                    );
                },
                "Quick Toast (1s)"
            }

            button {
                class: "w-full px-4 py-2 bg-teal-600 text-white rounded-md hover:bg-teal-700 transition-colors",
                onclick: move |_| {
                    toast_api.warning(
                        "Long Duration".to_string(),
                        Some(ToastOptions {
                            description: Some("This toast will stay visible for 10 seconds.".to_string()),
                            duration: Some(Duration::from_secs(10)),
                            ..Default::default()
                        }),
                    );
                },
                "Long Toast (10s)"
            }

            button {
                class: "w-full px-4 py-2 bg-orange-600 text-white rounded-md hover:bg-orange-700 transition-colors",
                onclick: move |_| {
                    toast_api.error(
                        "Critical Error".to_string(),
                        Some(ToastOptions {
                            description: Some("This is a critical error that requires immediate attention.".to_string()),
                            permanent: true,
                            ..Default::default()
                        }),
                    );
                },
                "Critical Error"
            }

            button {
                class: "w-full px-4 py-2 bg-slate-600 text-white rounded-md hover:bg-slate-700 transition-colors",
                onclick: move |_| {
                    toast_api.info(
                        "Rich Content".to_string(),
                        Some(ToastOptions {
                            description: Some("This toast contains rich content with detailed information about the current operation status.".to_string()),
                            duration: Some(Duration::from_secs(6)),
                            ..Default::default()
                        }),
                    );
                },
                "Rich Content"
            }
        }
    }
}
