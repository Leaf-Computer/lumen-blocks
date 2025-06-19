use dioxus::prelude::*;
use laminar_blocks::components::{
    avatar::{Avatar, AvatarImage, AvatarFallback},
    button::{Button, ButtonVariant, ButtonSize},
    progress::{Progress, ProgressSize, ProgressVariant},
};
use lucide_dioxus::{Check, X, Mail};

#[component]
pub fn StatsCard(
    title: String,
    value: String,
    change: String,
    positive: bool,
    icon: Element
) -> Element {
    rsx! {
        div { class: "bg-card rounded-lg border border-border p-6",
            div { class: "flex items-center justify-between",
                div {
                    p { class: "text-sm text-muted-foreground", "{title}" }
                    p { class: "text-2xl font-bold text-foreground mt-1", "{value}" }
                    p { 
                        class: if positive { "text-green-600 text-sm mt-1" } else { "text-red-600 text-sm mt-1" },
                        "{change} from last month"
                    }
                }
                div { class: "text-primary", {icon} }
            }
        }
    }
}

#[component]
pub fn ProjectCard(
    name: String,
    status: String,
    progress: f64,
    team_size: i32,
    due_date: String
) -> Element {
    let status_color = match status.as_str() {
        "Complete" => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
        "In Progress" => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
        "Planning" => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
        "Review" => "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
        _ => "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200",
    };
    
    rsx! {
        div { class: "flex items-center justify-between p-4 border border-border rounded-lg hover:bg-muted/50 transition-colors",
            div { class: "flex-1",
                div { class: "flex items-center gap-3 mb-2",
                    h4 { class: "font-medium text-foreground", "{name}" }
                    span { 
                        class: "px-2 py-1 text-xs font-medium rounded-full {status_color}",
                        "{status}"
                    }
                }
                div { class: "flex items-center gap-4 text-sm text-muted-foreground",
                    span { "Due: {due_date}" }
                    span { "Team: {team_size} members" }
                }
                div { class: "mt-3",
                    Progress {
                        value: Some(use_signal(|| progress).into()),
                        size: ProgressSize::Small,
                        variant: ProgressVariant::Default,
                    }
                }
            }
            Button {
                variant: use_signal(|| ButtonVariant::Ghost),
                is_icon_button: use_signal(|| true),
                aria_label: Some("More options".to_string()),
                X { class: "w-4 h-4" }
            }
        }
    }
}

#[component]
pub fn ProjectDetailCard(
    name: String,
    description: String,
    progress: f64,
    status: String,
    team: Vec<String>,
    tasks_completed: i32,
    tasks_total: i32,
    priority: String
) -> Element {
    let priority_color = match priority.as_str() {
        "High" => "text-red-600 bg-red-50 dark:bg-red-900/20 dark:text-red-400",
        "Medium" => "text-yellow-600 bg-yellow-50 dark:bg-yellow-900/20 dark:text-yellow-400",
        "Low" => "text-green-600 bg-green-50 dark:bg-green-900/20 dark:text-green-400",
        _ => "text-gray-600 bg-gray-50 dark:bg-gray-900/20 dark:text-gray-400",
    };
    
    let additional_members = format!("+{}", team.len() - 3);
    
    rsx! {
        div { class: "bg-card rounded-lg border border-border p-6 hover:shadow-md transition-shadow",
            div { class: "flex items-start justify-between mb-4",
                div { class: "flex-1",
                    h4 { class: "font-semibold text-foreground mb-1", "{name}" }
                    p { class: "text-sm text-muted-foreground", "{description}" }
                }
                span { 
                    class: "px-2 py-1 text-xs font-medium rounded-full {priority_color}",
                    "{priority}"
                }
            }
            
            div { class: "space-y-4",
                div {
                    div { class: "flex items-center justify-between text-sm mb-2",
                        span { class: "text-muted-foreground", "Progress" }
                        span { class: "text-foreground font-medium", "{progress:.0}%" }
                    }
                    Progress {
                        value: Some(use_signal(|| progress).into()),
                        size: ProgressSize::Medium,
                        variant: if progress >= 80.0 { ProgressVariant::Success } else { ProgressVariant::Default },
                    }
                }
                
                div { class: "flex items-center justify-between text-sm",
                    span { class: "text-muted-foreground", "Tasks" }
                    span { class: "text-foreground", "{tasks_completed}/{tasks_total}" }
                }
                
                div {
                    p { class: "text-sm text-muted-foreground mb-2", "Team Members" }
                    div { class: "flex -space-x-2",
                        for (_i, member) in team.iter().enumerate() {
                            Avatar {
                                class: Some("w-8 h-8 border-2 border-background".to_string()),
                                AvatarFallback { 
                                    class: Some("text-xs".to_string()),
                                    "{member.chars().take(2).collect::<String>().to_uppercase()}"
                                }
                            }
                        }
                        if team.len() > 3 {
                            div { class: "w-8 h-8 bg-muted border-2 border-background rounded-full flex items-center justify-center",
                                span { class: "text-xs text-muted-foreground", "{additional_members}" }
                            }
                        }
                    }
                }
                
                div { class: "flex gap-2 pt-2",
                    Button {
                        variant: use_signal(|| ButtonVariant::Outline),
                        size: use_signal(|| ButtonSize::Small),
                        full_width: use_signal(|| true),
                        icon_left: rsx! { Check { class: "w-3 h-3" } },
                        "Edit"
                    }
                    Button {
                        variant: use_signal(|| ButtonVariant::Ghost),
                        size: use_signal(|| ButtonSize::Small),
                        is_icon_button: use_signal(|| true),
                        aria_label: Some("Delete project".to_string()),
                        X { class: "w-3 h-3" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TeamMemberCard(
    name: String,
    role: String,
    email: String,
    avatar_url: String,
    status: String,
    projects: i32
) -> Element {
    let status_color = match status.as_str() {
        "Online" => "bg-green-400",
        "Away" => "bg-yellow-400",
        "Offline" => "bg-gray-400",
        _ => "bg-gray-400",
    };
    
    rsx! {
        div { class: "bg-card rounded-lg border border-border p-6",
            div { class: "flex items-center gap-4 mb-4",
                div { class: "relative",
                    Avatar {
                        class: Some("w-12 h-12".to_string()),
                        AvatarImage {
                            src: avatar_url,
                            alt: format!("{} Avatar", name),
                        }
                        AvatarFallback { 
                            "{name.chars().take(2).collect::<String>().to_uppercase()}"
                        }
                    }
                    div { class: "absolute -bottom-1 -right-1 w-4 h-4 {status_color} border-2 border-background rounded-full" }
                }
                div { class: "flex-1",
                    h4 { class: "font-semibold text-foreground", "{name}" }
                    p { class: "text-sm text-muted-foreground", "{role}" }
                    p { class: "text-xs text-muted-foreground", "{email}" }
                }
            }
            
            div { class: "space-y-3",
                div { class: "flex items-center justify-between text-sm",
                    span { class: "text-muted-foreground", "Status" }
                    span { class: "text-foreground capitalize", "{status}" }
                }
                div { class: "flex items-center justify-between text-sm",
                    span { class: "text-muted-foreground", "Active Projects" }
                    span { class: "text-foreground", "{projects}" }
                }
                
                div { class: "flex gap-2 pt-2",
                    Button {
                        variant: use_signal(|| ButtonVariant::Outline),
                        size: use_signal(|| ButtonSize::Small),
                        full_width: use_signal(|| true),
                        icon_left: rsx! { Mail { class: "w-3 h-3" } },
                        "Message"
                    }
                    Button {
                        variant: use_signal(|| ButtonVariant::Ghost),
                        size: use_signal(|| ButtonSize::Small),
                        is_icon_button: use_signal(|| true),
                        aria_label: Some("More options".to_string()),
                        X { class: "w-3 h-3" }
                    }
                }
            }
        }
    }
}
