use dioxus_lib::prelude::*;
use dioxus_primitives::progress::Progress as ProgressPrimitive;

/// Progress size variants
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProgressSize {
    Small,
    Medium,
    Large,
}

impl Default for ProgressSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Progress color variants
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProgressVariant {
    Default,
    Destructive,
    Success,
    Warning,
}

impl Default for ProgressVariant {
    fn default() -> Self {
        Self::Default
    }
}

/// Props for the Progress component
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// The current progress value, between 0 and max
    value: ReadOnlySignal<f64>,

    /// The maximum value. Defaults to 100
    #[props(default = 100.0)]
    max: f64,
    
    /// Size variant of the progress bar
    #[props(default)]
    pub size: ProgressSize,
    
    /// Color variant of the progress bar
    #[props(default)]
    pub variant: ProgressVariant,
    
    /// Optional ID for the progress element
    #[props(default)]
    pub id: Option<String>,
    
    /// Accessible label for the progress
    #[props(default)]
    pub aria_label: Option<String>,
    
    /// Whether to show the percentage text
    #[props(default = false)]
    pub show_percentage: bool,
    
    /// Custom class names for the progress container
    #[props(default)]
    pub class: Option<String>,
}

/// A styled progress component for showing completion progress
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    // Calculate percentage
    let current: f64 = (props.value)();
    let max_value = props.max;
    let percentage = (current / max_value * 100.0).min(100.0).max(0.0);

    // Determine size-specific classes
    let height_class = match props.size {
        ProgressSize::Small => "h-2",
        ProgressSize::Medium => "h-3",
        ProgressSize::Large => "h-4",
    };

    // Determine color variant classes
    let indicator_color = match props.variant {
        ProgressVariant::Default => "bg-primary",
        ProgressVariant::Destructive => "bg-destructive",
        ProgressVariant::Success => "bg-green-500",
        ProgressVariant::Warning => "bg-yellow-500",
    };

    // Build container classes
    let container_class = format!(
        "relative w-full overflow-hidden rounded-full bg-secondary {}",
        height_class
    );

    let combined_class = if let Some(custom_class) = &props.class {
        format!("{} {}", container_class, custom_class)
    } else {
        container_class
    };

    // Build indicator classes
    let indicator_class = format!(
        "h-full transition-all duration-300 ease-in-out {}",
        indicator_color
    );

    rsx! {
        div { class: "w-full space-y-2",
            if props.show_percentage {
                div { class: "flex justify-between text-sm text-muted-foreground",
                    span { 
                        if let Some(label) = &props.aria_label {
                            "{label}"
                        } else {
                            "Progress"
                        }
                    }
                    span { "{percentage:.0}%" }
                }
            }

            ProgressPrimitive {
                value: props.value,
                max: props.max,
                class: combined_class,
                id: props.id.clone(),

                div {
                    class: indicator_class,
                    style: format!("width: {}%", percentage),
                }
            }
        }
    }
}
