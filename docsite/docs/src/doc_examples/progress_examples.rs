#![allow(non_snake_case)]

pub use basic::BasicProgressExample;
pub use sizes::ProgressSizesExample;
pub use variants::ProgressVariantsExample;
pub use interactive::InteractiveProgressExample;
pub use percentages::ProgressWithPercentageExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::progress::Progress;
    
    #[component]
    pub fn BasicProgressExample() -> Element {
        let progress: Signal<f64> = use_signal(|| 65.0);
        
        rsx! {
            div { class: "space-y-4",
                div { class: "space-y-2",
                    label { class: "text-sm font-medium",
                        "Simple Progress Bar"
                    }
                    Progress {
                        value: progress,
                        max: 100.0,
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
    use laminar_blocks::components::progress::Progress;
    
    #[component]
    pub fn ProgressWithPercentageExample() -> Element {
        let progress: Signal<f64> = use_signal(|| 80.0);
        
        rsx! {
            div { class: "space-y-4",
                div { class: "space-y-2",
                    label { class: "text-sm font-medium",
                        "With Percentage Display"
                    }
                    Progress {
                        value: progress,
                        max: 100.0,
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
    use laminar_blocks::components::progress::Progress;
    
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
                        value: progress_value,
                        max: 100.0,
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
    use laminar_blocks::components::progress::{Progress, ProgressSize};
    
    #[component]
    pub fn ProgressSizesExample() -> Element {
        let progress_value = use_signal(|| 45.0);
        
        rsx! {
            div { class: "space-y-4",
                div { class: "space-y-2",
                    label { class: "text-sm font-medium",
                        "Small Size"
                    }
                    Progress {
                        value: progress_value,
                        max: 100.0,
                        size: ProgressSize::Small,
                        show_percentage: true,
                    }
                }
                
                div { class: "space-y-2",
                    label { class: "text-sm font-medium",
                        "Medium Size (Default)"
                    }
                    Progress {
                        value: progress_value,
                        max: 100.0,
                        size: ProgressSize::Medium,
                        show_percentage: true,
                    }
                }
                
                div { class: "space-y-2",
                    label { class: "text-sm font-medium",
                        "Large Size"
                    }
                    Progress {
                        value: progress_value,
                        max: 100.0,
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
    use laminar_blocks::components::progress::{Progress, ProgressVariant};
    
    #[component]
    pub fn ProgressVariantsExample() -> Element {
        let progress_value = use_signal(|| 45.0);
        
        rsx! {
            div { class: "space-y-4",
                div { class: "space-y-2",
                    label { class: "text-sm font-medium",
                        "Default"
                    }
                    Progress {
                        value: progress_value,
                        max: 100.0,
                        variant: ProgressVariant::Default,
                        show_percentage: true,
                    }
                }
                
                div { class: "space-y-2",
                    label { class: "text-sm font-medium text-green-600",
                        "Success"
                    }
                    Progress {
                        value: progress_value,
                        max:100.0,
                        variant: ProgressVariant::Success,
                        show_percentage: true,
                    }
                }
                
                div { class: "space-y-2",
                    label { class: "text-sm font-medium text-yellow-600",
                        "Warning"
                    }
                    Progress {
                        value: progress_value,
                        max: 100.0,
                        variant: ProgressVariant::Warning,
                        show_percentage: true,
                    }
                }
                
                div { class: "space-y-2",
                    label { class: "text-sm font-medium text-red-600",
                        "Destructive"
                    }
                    Progress {
                        value: progress_value,
                        max: 100.0,
                        variant: ProgressVariant::Destructive,
                        show_percentage: true,
                    }
                }
            }
        }
    }
    // ANCHOR_END: variants
}
