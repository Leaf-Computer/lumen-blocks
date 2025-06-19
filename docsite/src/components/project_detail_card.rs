use dioxus::prelude::*;
use laminar_blocks::components::{
    progress::{Progress, ProgressVariant},
    avatar::{Avatar, AvatarFallback},
    button::{Button, ButtonVariant},
};
use lucide_dioxus::{Clock, Users, Check};

#[component]
pub fn ProjectDetailCard(
    name: String,
    description: String,
    progress: f64,
    status: String,
    team: Vec<String>,
    tasks_completed: u32,
    tasks_total: u32,
    priority: String,
) -> Element {
    // Determine progress variant based on progress value
    let progress_variant = if progress >= 80.0 {
        ProgressVariant::Success
    } else if progress >= 40.0 {
        ProgressVariant::Default
    } else {
        ProgressVariant::Warning
    };
    
    // Determine priority badge color
    let priority_class = match priority.to_lowercase().as_str() {
        "high" => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
        "medium" => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
        "low" => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
        _ => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
    };
    
    // Determine status badge color
    let status_class = match status.to_lowercase().as_str() {
        "completed" => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
        "in progress" => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
        "planning" => "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
        "review" => "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200",
        _ => "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200",
    };

    rsx! {
        div { class: "bg-card rounded-lg border border-border overflow-hidden shadow-sm",
            // Header
            div { class: "p-6 border-b border-border",
                div { class: "flex items-center justify-between mb-2",
                    h3 { class: "text-lg font-semibold text-foreground", "{name}" }
                    div { class: "flex gap-2",
                        span { class: "px-2.5 py-0.5 rounded-full text-xs font-medium {status_class}", "{status}" }
                        span { class: "px-2.5 py-0.5 rounded-full text-xs font-medium {priority_class}", "{priority} Priority" }
                    }
                }
                p { class: "text-sm text-muted-foreground", "{description}" }
            }
            
            // Progress
            div { class: "px-6 py-4 border-b border-border",
                div { class: "flex justify-between mb-1 text-sm",
                    span { class: "font-medium", "Progress" }
                    span { "{progress as u32}%" }
                }
                Progress {
                    value: Some(use_signal(|| progress).into()),
                    variant: progress_variant,
                }
            }
            
            // Stats
            div { class: "grid grid-cols-3 divide-x divide-border border-b border-border",
                div { class: "p-4 text-center",
                    div { class: "flex flex-col items-center",
                        Users { class: "w-5 h-5 text-muted-foreground mb-1" }
                        span { class: "text-sm font-medium", "{team.len()}" }
                        span { class: "text-xs text-muted-foreground", "Team Members" }
                    }
                }
                div { class: "p-4 text-center",
                    div { class: "flex flex-col items-center",
                        Check { class: "w-5 h-5 text-muted-foreground mb-1" }
                        span { class: "text-sm font-medium", "{tasks_completed}/{tasks_total}" }
                        span { class: "text-xs text-muted-foreground", "Tasks" }
                    }
                }
                div { class: "p-4 text-center",
                    div { class: "flex flex-col items-center",
                        Clock { class: "w-5 h-5 text-muted-foreground mb-1" }
                        span { class: "text-sm font-medium", 
                            if progress < 100.0 { "Active" } else { "Complete" }
                        }
                        span { class: "text-xs text-muted-foreground", "Status" }
                    }
                }
            }
            
            // Team
            div { class: "p-6",
                h4 { class: "text-sm font-medium mb-3", "Team Members" }
                div { class: "flex -space-x-2 overflow-hidden",
                    for (_index, member) in team.iter().enumerate().take(4) {
                        Avatar {
                            class: Some("border-2 border-background".to_string()),
                            AvatarFallback { 
                                "{member.split_whitespace().map(|word| word.chars().next().unwrap_or(' ')).collect::<String>()}" 
                            }
                        }
                    }
                    
                    if team.len() > 4 {
                        div { 
                            class: "flex items-center justify-center w-8 h-8 rounded-full bg-muted text-xs font-medium text-muted-foreground border-2 border-background",
                            "+{team.len() - 4}"
                        }
                    }
                }
            }
            
            // Actions
            div { class: "px-6 py-4 bg-muted/50 border-t border-border flex justify-between",
                Button {
                    variant: use_signal(|| ButtonVariant::Ghost),
                    "View Details"
                }
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    "Edit Project"
                }
            }
        }
    }
}
