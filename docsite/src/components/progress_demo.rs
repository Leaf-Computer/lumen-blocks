use dioxus::prelude::*;
use laminar_blocks::components::{
    progress::{Progress, ProgressVariant, ProgressSize},
    button::{Button, ButtonVariant},
};
use lucide_dioxus::{RefreshCw, Play, X};
use dioxus_time::use_timeout;

#[component]
pub fn ProgressDemo() -> Element {
    let mut progress_value = use_signal(|| 30.0);
    let mut is_running = use_signal(|| false);
    
    // Create an effect to increment the progress when running
    use_effect(move || {
        if is_running() && progress_value() < 100.0 {
            let handle = use_timeout(std::time::Duration::from_millis(200), move |_: String| {
                if progress_value() < 100.0 {
                    progress_value.set(progress_value() + 1.0);
                } else {
                    is_running.set(false);
                }
            });
            
            return;
        }
        return
    });
    
    let toggle_progress = move |_| {
        is_running.set(!is_running());
    };
    
    let reset_progress = move |_| {
        progress_value.set(0.0);
        is_running.set(false);
    };

    rsx! {
        div { class: "p-6 bg-card rounded-lg shadow-sm border border-border",
            h3 { class: "text-xl font-semibold mb-4", "Progress Indicators" }
            p { class: "text-muted-foreground mb-6", 
                "Progress indicators help users understand the current status of an operation or task."
            }
            
            div { class: "space-y-8",
                // Basic progress bar
                div { class: "space-y-3",
                    h4 { class: "font-medium", "Standard Progress Bar" }
                    Progress {
                        value: Some(progress_value.into()),
                        max: ReadOnlySignal::new(Signal::new(100.0)),
                    }
                    p { class: "text-sm text-muted-foreground", 
                        "Current progress: {progress_value()}%" 
                    }
                }
                
                // Indeterminate progress
                div { class: "space-y-3",
                    h4 { class: "font-medium", "Indeterminate Progress" }
                    Progress {
                        value: None,
                        max: ReadOnlySignal::new(Signal::new(100.0)),
                    }
                    p { class: "text-sm text-muted-foreground", 
                        "Used when progress cannot be determined" 
                    }
                }
                
                // Progress with custom indicator
                div { class: "space-y-3",
                    h4 { class: "font-medium", "Custom Indicator" }
                    Progress {
                        value: Some(progress_value.into()),
                        max: ReadOnlySignal::new(Signal::new(100.0)),
                        variant: ProgressVariant::Success,
                    }
                    p { class: "text-sm text-muted-foreground", 
                        "Custom colored indicator" 
                    }
                }
                
                // Progress with labels
                div { class: "space-y-3",
                    h4 { class: "font-medium", "Progress with Labels" }
                    div { class: "flex justify-between text-sm mb-1",
                        span { "Downloading..." }
                        span { "{progress_value()}%" }
                    }
                    Progress {
                        value: Some(progress_value.into()),
                        max: ReadOnlySignal::new(Signal::new(100.0)),
                        show_percentage: false,
                    }
                }
                
                // Controls
                div { class: "flex gap-4 mt-6",
                    Button {
                        variant: use_signal(|| if is_running() { ButtonVariant::Secondary } else { ButtonVariant::Primary }),
                        icon_left: rsx! { 
                            if is_running() {
                                X { class: "w-4 h-4" }
                            } else {
                                Play { class: "w-4 h-4" }
                            }
                        },
                        on_click: toggle_progress,
                        if is_running() { "Pause" } else { "Start" }
                    }
                    
                    Button {
                        variant: use_signal(|| ButtonVariant::Outline),
                        icon_left: rsx! { RefreshCw { class: "w-4 h-4" } },
                        on_click: reset_progress,
                        "Reset"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProgressSection() -> Element {
    rsx! {
        section { class: "mb-10",
            h2 { class: "text-2xl font-bold mb-6", "Progress Indicators" }
            div { class: "space-y-6",
                ProgressDemo {}
                
                div { class: "mt-6 p-4 bg-muted rounded-lg",
                    h4 { class: "font-medium mb-2", "Usage Notes" }
                    ul { class: "list-disc pl-5 space-y-1 text-sm text-muted-foreground",
                        li { "Use progress indicators for operations that take time to complete." }
                        li { "The indeterminate progress bar is useful when progress cannot be calculated." }
                        li { "Provide context with labels when appropriate." }
                        li { "Consider using different colors for different states (success, warning, etc.)." }
                    }
                }
            }
        }
    }
}
