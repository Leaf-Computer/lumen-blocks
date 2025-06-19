use dioxus::prelude::*;
use laminar_blocks::components::{
    button::{Button, ButtonVariant},
    toast::{use_toast, ToastOptions, ToastProvider},
};
use lucide_dioxus::{TriangleAlert, Check, Info, X};
use std::time::Duration;

#[component]
pub fn ToastDemo() -> Element {
    let toasts = use_toast();
    
    let trigger_success_toast = move |_| {
        toasts.success(
            "Success".to_string(),
            Some(ToastOptions {
                description: Some("Your action has been completed successfully.".to_string()),
                duration: Some(Duration::from_secs(3)),
                ..Default::default()
            })
        );
    };
    
    let trigger_error_toast = move |_| {
        toasts.error(
            "Error".to_string(),
            Some(ToastOptions {
                description: Some("There was an error processing your request.".to_string()),
                duration: Some(Duration::from_secs(3)),
                ..Default::default()
            })
        );
    };
    
    let trigger_info_toast = move |_| {
        toasts.info(
            "Information".to_string(),
            Some(ToastOptions {
                description: Some("This is an informational message for you.".to_string()),
                duration: Some(Duration::from_secs(3)),
                ..Default::default()
            })
        );
    };
    
    let trigger_warning_toast = move |_| {
        toasts.warning(
            "Warning".to_string(),
            Some(ToastOptions {
                description: Some("Please be cautious with this action.".to_string()),
                duration: Some(Duration::from_secs(3)),
                ..Default::default()
            })
        );
    };

    rsx! {
        div { class: "p-6 bg-card rounded-lg shadow-sm border border-border",
            h3 { class: "text-xl font-semibold mb-4", "Toast Notifications" }
            p { class: "text-muted-foreground mb-6", 
                "Toast notifications provide feedback about operations without getting in the way of the user experience."
            }
            
            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                div {
                    Button {
                        variant: use_signal(|| ButtonVariant::Primary),
                        icon_left: rsx! { Check { class: "w-4 h-4" } },
                        on_click: trigger_success_toast,
                        "Success Toast"
                    }
                }
                
                div {
                    Button {
                        variant: use_signal(|| ButtonVariant::Destructive),
                        icon_left: rsx! { X { class: "w-4 h-4" } },
                        on_click: trigger_error_toast,
                        "Error Toast"
                    }
                }
                
                div {
                    Button {
                        variant: use_signal(|| ButtonVariant::Outline),
                        icon_left: rsx! { Info { class: "w-4 h-4" } },
                        on_click: trigger_info_toast,
                        "Info Toast"
                    }
                }
                
                div {
                    Button {
                        variant: use_signal(|| ButtonVariant::Secondary),
                        icon_left: rsx! { TriangleAlert { class: "w-4 h-4" } },
                        on_click: trigger_warning_toast,
                        "Warning Toast"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ToastSection() -> Element {
    rsx! {
        ToastProvider {
            default_duration: Duration::from_secs(4),
            max_toasts: 5,
            
            section { class: "mb-10",
                h2 { class: "text-2xl font-bold mb-6", "Toast Notifications" }
                div { class: "space-y-6",
                    ToastDemo {}
                    
                    div { class: "mt-6 p-4 bg-muted rounded-lg",
                        h4 { class: "font-medium mb-2", "Usage Notes" }
                        ul { class: "list-disc pl-5 space-y-1 text-sm text-muted-foreground",
                            li { "Toast notifications are non-modal and don't block user interaction." }
                            li { "They automatically disappear after a timeout unless configured otherwise." }
                            li { "Actions can be added to toast notifications for quick responses." }
                            li { "Different variants communicate different types of messages." }
                        }
                    }
                }
            }
        }
    }
}
