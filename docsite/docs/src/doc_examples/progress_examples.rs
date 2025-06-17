#![allow(non_snake_case)]

pub use basic::BasicProgressExample;
pub use sizes::ProgressSizesExample;
pub use variants::ProgressVariantsExample;
pub use interactive::InteractiveProgressExample;
pub use percentages::ProgressWithPercentageExample;
pub use real_world::RealWorldProgressExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::progress::{Progress, ProgressSize, ProgressVariant};
    
    #[component]
    pub fn BasicProgressExample() -> Element {
        rsx! {
            div { class: "space-y-4",
                div { class: "space-y-2",
                    label { class: "text-sm font-medium",
                        "Simple Progress Bar"
                    }
                    Progress {
                        value: Some(ReadOnlySignal::new(Signal::new(65.0))),
                        max: ReadOnlySignal::new(Signal::new(100.0)),
                    }
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod percentages {
    // ANCHOR: percentages
    use dioxus::prelude::*;
    use laminar_blocks::components::progress::{Progress, ProgressSize, ProgressVariant};
    
    #[component]
    pub fn ProgressWithPercentageExample() -> Element {
        rsx! {
            div { class: "space-y-4",
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
    }
    // ANCHOR_END: percentages
}

pub mod interactive {
    // ANCHOR: interactive
    use dioxus::prelude::*;
    use laminar_blocks::components::progress::{Progress, ProgressSize, ProgressVariant};
    
    #[component]
    pub fn InteractiveProgressExample() -> Element {
        let mut progress_value = use_signal(|| 45.0);

        rsx! {
            div { class: "space-y-4",
                div { class: "space-y-2",
                    label { class: "text-sm font-medium",
                        "Manual Control - {progress_value()}%"
                    }
                    Progress {
                        value: Some(progress_value.into()),
                        max: ReadOnlySignal::new(Signal::new(100.0)),
                        show_percentage: true,
                    }
                    
                    div { class: "flex gap-2 mt-2",
                        button { 
                            class: "px-3 py-1 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors text-sm",
                            onclick: move |_| {
                                progress_value.set(f64::max(progress_value() - 10.0, 0.0));
                            },
                            disabled: progress_value() <= 0.0,
                            "- 10%"
                        }
                        button { 
                            class: "px-3 py-1 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors text-sm",
                            onclick: move |_| {
                                progress_value.set(f64::min(progress_value() + 10.0, 100.0));
                            },
                            disabled: progress_value() >= 100.0,
                            "+ 10%"
                        }
                        button { 
                            class: "px-3 py-1 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors text-sm",
                            onclick: move |_| {
                                progress_value.set(0.0);
                            },
                            "Reset"
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: interactive
}

pub mod sizes {
    // ANCHOR: sizes
    use dioxus::prelude::*;
    use laminar_blocks::components::progress::{Progress, ProgressSize, ProgressVariant};
    
    #[component]
    pub fn ProgressSizesExample() -> Element {
        rsx! {
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
    }
    // ANCHOR_END: sizes
}

pub mod variants {
    // ANCHOR: variants
    use dioxus::prelude::*;
    use laminar_blocks::components::progress::{Progress, ProgressSize, ProgressVariant};
    
    #[component]
    pub fn ProgressVariantsExample() -> Element {
        rsx! {
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
    }
    // ANCHOR_END: variants
}

pub mod real_world {
    // ANCHOR: real_world
    use dioxus::prelude::*;
    use laminar_blocks::components::progress::{Progress, ProgressSize, ProgressVariant};
    
    #[component]
    pub fn RealWorldProgressExample() -> Element {
        let mut upload_progress = use_signal(|| 75.0);

        rsx! {
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
    // ANCHOR_END: real_world
}