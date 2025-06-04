use dioxus::prelude::*;
use dioxus_blocks::components::progress::{Progress, ProgressSize, ProgressVariant};

#[component]
pub fn ProgressExample() -> Element {
    let mut basic_progress = use_signal(|| 45.0);
    let loading_progress = use_signal(|| 33.0);
    let mut upload_progress = use_signal(|| 75.0);

    rsx! {
        div { class: "p-6 my-6 max-w-4xl mx-auto bg-background rounded-xl shadow-md text-foreground border border-border",
            h2 { class: "text-2xl font-bold mb-6",
                "Progress Examples"
            }
            
            // Basic Progress
            div { class: "space-y-4",
                h3 { class: "text-lg font-semibold mb-4",
                    "Basic Progress"
                }
                
                div { class: "space-y-3",
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium",
                            "Simple Progress Bar"
                        }
                        Progress {
                            value: Some(ReadOnlySignal::new(Signal::new(65.0))),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                        }
                    }
                    
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium",
                            "With Percentage Display"
                        }
                        Progress {
                            value: Some(ReadOnlySignal::new(Signal::new(80.0))),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            show_percentage: true,
                            aria_label: Some("Download Progress".to_string()),
                        }
                    }
                }
            }
            
            // Interactive Progress
            div { class: "mt-8 space-y-4",
                h3 { class: "text-lg font-semibold mb-4",
                    "Interactive Progress"
                }
                
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium",
                            "Manual Control - {basic_progress()}%"
                        }
                        Progress {
                            value: Some(basic_progress.into()),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            show_percentage: true,
                        }
                        
                        div { class: "flex gap-2 mt-2",
                            button { 
                                class: "px-3 py-1 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors text-sm",
                                onclick: move |_| {
                                    basic_progress.set(f64::max(basic_progress() - 10.0, 0.0));
                                },
                                disabled: basic_progress() <= 0.0,
                                "- 10%"
                            }
                            button { 
                                class: "px-3 py-1 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors text-sm",
                                onclick: move |_| {
                                    basic_progress.set(f64::min(basic_progress() + 10.0, 100.0));
                                },
                                disabled: basic_progress() >= 100.0,
                                "+ 10%"
                            }
                            button { 
                                class: "px-3 py-1 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors text-sm",
                                onclick: move |_| {
                                    basic_progress.set(0.0);
                                },
                                "Reset"
                            }
                        }
                    }
                    
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium",
                            "Static Loading Progress"
                        }
                        Progress {
                            value: Some(loading_progress.into()),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            show_percentage: true,
                            aria_label: Some("Loading Progress".to_string()),
                        }
                    }
                }
            }
            
            // Size Variants
            div { class: "mt-8 space-y-4",
                h3 { class: "text-lg font-semibold mb-4",
                    "Size Variants"
                }
                
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium",
                            "Small Size"
                        }
                        Progress {
                            value: Some(ReadOnlySignal::new(Signal::new(40.0))),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            size: ProgressSize::Small,
                            show_percentage: true,
                        }
                    }
                    
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium",
                            "Medium Size (Default)"
                        }
                        Progress {
                            value: Some(ReadOnlySignal::new(Signal::new(60.0))),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            size: ProgressSize::Medium,
                            show_percentage: true,
                        }
                    }
                    
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium",
                            "Large Size"
                        }
                        Progress {
                            value: Some(ReadOnlySignal::new(Signal::new(80.0))),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            size: ProgressSize::Large,
                            show_percentage: true,
                        }
                    }
                }
            }
            
            // Color Variants
            div { class: "mt-8 space-y-4",
                h3 { class: "text-lg font-semibold mb-4",
                    "Color Variants"
                }
                
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium",
                            "Default"
                        }
                        Progress {
                            value: Some(ReadOnlySignal::new(Signal::new(50.0))),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            variant: ProgressVariant::Default,
                            show_percentage: true,
                        }
                    }
                    
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium text-green-600",
                            "Success"
                        }
                        Progress {
                            value: Some(ReadOnlySignal::new(Signal::new(100.0))),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            variant: ProgressVariant::Success,
                            show_percentage: true,
                        }
                    }
                    
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium text-yellow-600",
                            "Warning"
                        }
                        Progress {
                            value: Some(ReadOnlySignal::new(Signal::new(75.0))),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            variant: ProgressVariant::Warning,
                            show_percentage: true,
                        }
                    }
                    
                    div { class: "space-y-2",
                        label { class: "text-sm font-medium text-red-600",
                            "Destructive"
                        }
                        Progress {
                            value: Some(ReadOnlySignal::new(Signal::new(25.0))),
                            max: ReadOnlySignal::new(Signal::new(100.0)),
                            variant: ProgressVariant::Destructive,
                            show_percentage: true,
                        }
                    }
                }
            }
            
            // Real-world Examples
            div { class: "mt-8 space-y-4",
                h3 { class: "text-lg font-semibold mb-4",
                    "Real-world Examples"
                }
                
                div { class: "space-y-6",
                    // File Upload Example
                    div { class: "p-4 bg-secondary/20 rounded-lg border",
                        h4 { class: "text-md font-medium mb-3",
                            "File Upload Progress"
                        }
                        
                        div { class: "space-y-2",
                            div { class: "flex justify-between text-sm",
                                span { "document.pdf" }
                                span { "2.4 MB / 3.2 MB" }
                            }
                            Progress {
                                value: Some(upload_progress.into()),
                                max: ReadOnlySignal::new(Signal::new(100.0)),
                                variant: ProgressVariant::Default,
                                show_percentage: true,
                            }
                            
                            div { class: "flex gap-2 mt-3",
                                button { 
                                    class: "px-3 py-1 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors text-sm",
                                    onclick: move |_| {
                                        upload_progress.set(f64::min(upload_progress() + 5.0, 100.0));
                                    },
                                    disabled: upload_progress() >= 100.0,
                                    if upload_progress() >= 100.0 { "Completed" } else { "Continue Upload" }
                                }
                                button { 
                                    class: "px-3 py-1 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors text-sm",
                                    onclick: move |_| {
                                        upload_progress.set(0.0);
                                    },
                                    "Reset"
                                }
                            }
                        }
                    }
                    
                    // Task Completion Example
                    div { class: "p-4 bg-secondary/20 rounded-lg border",
                        h4 { class: "text-md font-medium mb-3",
                            "Task Completion"
                        }
                        
                        div { class: "space-y-3",
                            div { class: "space-y-2",
                                div { class: "flex justify-between text-sm",
                                    span { "Setup Project" }
                                    span { "✓ Complete" }
                                }
                                Progress {
                                    value: Some(ReadOnlySignal::new(Signal::new(100.0))),
                                    max: ReadOnlySignal::new(Signal::new(100.0)),
                                    variant: ProgressVariant::Success,
                                    size: ProgressSize::Small,
                                }
                            }
                            
                            div { class: "space-y-2",
                                div { class: "flex justify-between text-sm",
                                    span { "Install Dependencies" }
                                    span { "In Progress..." }
                                }
                                Progress {
                                    value: Some(ReadOnlySignal::new(Signal::new(60.0))),
                                    max: ReadOnlySignal::new(Signal::new(100.0)),
                                    variant: ProgressVariant::Default,
                                    size: ProgressSize::Small,
                                }
                            }
                            
                            div { class: "space-y-2",
                                div { class: "flex justify-between text-sm",
                                    span { "Run Tests" }
                                    span { "Pending" }
                                }
                                Progress {
                                    value: Some(ReadOnlySignal::new(Signal::new(0.0))),
                                    max: ReadOnlySignal::new(Signal::new(100.0)),
                                    variant: ProgressVariant::Default,
                                    size: ProgressSize::Small,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(ProgressExample);
}