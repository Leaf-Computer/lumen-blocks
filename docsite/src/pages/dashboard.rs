use dioxus::prelude::*;
use crate::components::{AppLayout, StatsCard, ProjectCard, ComponentShowcase};
use laminar_blocks::components::{
    button::{Button, ButtonVariant, ButtonSize},
    progress::{Progress, ProgressSize, ProgressVariant},
};
use lucide_dioxus::{Check, Info, X};

#[component]
pub fn Dashboard() -> Element {
    let current_section = use_signal(|| "overview".to_string());
    let notification_enabled = use_signal(|| true);
    let dark_mode = use_signal(|| false);
    let project_progress = use_signal(|| 65.0);
    
    rsx! {
        AppLayout {
            current_section,
            match current_section().as_str() {
                "overview" => rsx! { OverviewSection { project_progress } },
                "projects" => rsx! { ProjectsSection {} },
                "team" => rsx! { TeamSection {} },
                "components" => rsx! { 
                    ComponentShowcase { 
                        notification_enabled,
                        dark_mode 
                    } 
                },
                _ => rsx! { OverviewSection { project_progress } }
            }
        }
    }
}

#[component]
fn OverviewSection(mut project_progress: Signal<f64>) -> Element {
    rsx! {
        div { class: "space-y-6",
            // Stats Cards
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
                StatsCard {
                    title: "Total Projects".to_string(),
                    value: "24".to_string(),
                    change: "+12%".to_string(),
                    positive: true,
                    icon: rsx! { Info { class: "w-6 h-6" } }
                }
                StatsCard {
                    title: "Active Tasks".to_string(),
                    value: "156".to_string(),
                    change: "+5%".to_string(),
                    positive: true,
                    icon: rsx! { Check { class: "w-6 h-6" } }
                }
                StatsCard {
                    title: "Team Members".to_string(),
                    value: "12".to_string(),
                    change: "+2".to_string(),
                    positive: true,
                    icon: rsx! { X { class: "w-6 h-6" } }
                }
                StatsCard {
                    title: "Completion Rate".to_string(),
                    value: "87%".to_string(),
                    change: "+3%".to_string(),
                    positive: true,
                    icon: rsx! { Info { class: "w-6 h-6" } }
                }
            }
            
            // Recent Projects
            div { class: "bg-card rounded-lg border border-border p-6",
                div { class: "flex items-center justify-between mb-6",
                    h3 { class: "text-lg font-semibold text-foreground", "Recent Projects" }
                    Button {
                        variant: Signal::new(ButtonVariant::Outline),
                        size: Signal::new(ButtonSize::Small),
                        "View All"
                    }
                }
                
                div { class: "space-y-4",
                    ProjectCard {
                        name: "Website Redesign".to_string(),
                        status: "In Progress".to_string(),
                        progress: 75.0,
                        team_size: 5,
                        due_date: "Dec 15".to_string()
                    }
                    ProjectCard {
                        name: "Mobile App".to_string(),
                        status: "Planning".to_string(),
                        progress: 25.0,
                        team_size: 3,
                        due_date: "Jan 20".to_string()
                    }
                    ProjectCard {
                        name: "Database Migration".to_string(),
                        status: "Review".to_string(),
                        progress: 90.0,
                        team_size: 2,
                        due_date: "Dec 5".to_string()
                    }
                }
            }
            
            // Interactive Progress Demo
            div { class: "bg-card rounded-lg border border-border p-6",
                h3 { class: "text-lg font-semibold text-foreground mb-4", "Project Progress Demo" }
                p { class: "text-muted-foreground mb-4", "Adjust the progress slider to see the progress component in action:" }
                
                div { class: "space-y-4",
                    div { class: "flex items-center gap-4",
                        label { class: "text-sm font-medium text-foreground min-w-[100px]", "Current Progress:" }
                        input {
                            r#type: "range",
                            min: "0",
                            max: "100",
                            value: "{project_progress()}",
                            class: "flex-1",
                            oninput: move |e| {
                                if let Ok(value) = e.value().parse::<f64>() {
                                    project_progress.set(value);
                                }
                            }
                        }
                        span { class: "text-sm text-muted-foreground min-w-[40px]", "{project_progress():.0}%" }
                    }
                    
                    div { class: "space-y-3",
                        Progress {
                            value: Some(Signal::new(project_progress()).into()),
                            size: ProgressSize::Small,
                            variant: ProgressVariant::Default,
                            show_percentage: true,
                            aria_label: Some("Small Progress".to_string()),
                        }
                        Progress {
                            value: Some(Signal::new(project_progress()).into()),
                            size: ProgressSize::Medium,
                            variant: ProgressVariant::Success,
                            show_percentage: true,
                            aria_label: Some("Medium Progress".to_string()),
                        }
                        Progress {
                            value: Some(Signal::new(project_progress()).into()),
                            size: ProgressSize::Large,
                            variant: ProgressVariant::Warning,
                            show_percentage: true,
                            aria_label: Some("Large Progress".to_string()),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectsSection() -> Element {
    rsx! {
        div { class: "space-y-6",
            div { class: "flex items-center justify-between",
                div {
                    h3 { class: "text-lg font-semibold text-foreground", "Project Management" }
                    p { class: "text-muted-foreground", "Manage and track your projects" }
                }
                div { class: "flex gap-2",
                    Button {
                        variant: Signal::new(ButtonVariant::Outline),
                        icon_left: rsx! { Info { class: "w-4 h-4" } },
                        "Filter"
                    }
                    Button {
                        variant: Signal::new(ButtonVariant::Outline),
                        icon_left: rsx! { X { class: "w-4 h-4" } },
                        "Export"
                    }
                }
            }
            
            div { class: "grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6",
                crate::components::ProjectDetailCard {
                    name: "E-commerce Platform".to_string(),
                    description: "Building a modern e-commerce solution with React and Node.js".to_string(),
                    progress: 80.0,
                    status: "In Progress".to_string(),
                    team: vec!["Alice Johnson".to_string(), "Bob Smith".to_string(), "Carol Davis".to_string()],
                    tasks_completed: 24,
                    tasks_total: 30,
                    priority: "High".to_string()
                }
                crate::components::ProjectDetailCard {
                    name: "Marketing Website".to_string(),
                    description: "Redesigning company website with focus on conversion optimization".to_string(),
                    progress: 45.0,
                    status: "Planning".to_string(),
                    team: vec!["David Wilson".to_string(), "Eve Brown".to_string()],
                    tasks_completed: 12,
                    tasks_total: 28,
                    priority: "Medium".to_string()
                }
                crate::components::ProjectDetailCard {
                    name: "Data Analytics Dashboard".to_string(),
                    description: "Creating comprehensive analytics dashboard for business intelligence".to_string(),
                    progress: 95.0,
                    status: "Review".to_string(),
                    team: vec!["Frank Miller".to_string(), "Grace Lee".to_string(), "Henry Taylor".to_string()],
                    tasks_completed: 19,
                    tasks_total: 20,
                    priority: "High".to_string()
                }
            }
        }
    }
}

#[component]
fn TeamSection() -> Element {
    rsx! {
        div { class: "space-y-6",
            div { class: "flex items-center justify-between",
                div {
                    h3 { class: "text-lg font-semibold text-foreground", "Team Members" }
                    p { class: "text-muted-foreground", "Manage your team and permissions" }
                }
                Button {
                    variant: Signal::new(ButtonVariant::Primary),
                    icon_left: rsx! { Check { class: "w-4 h-4" } },
                    "Invite Member"
                }
            }
            
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                crate::components::TeamMemberCard {
                    name: "Alice Johnson".to_string(),
                    role: "Product Manager".to_string(),
                    email: "alice@company.com".to_string(),
                    avatar_url: "https://images.unsplash.com/photo-1494790108755-2616b612b786?w=80&h=80&fit=crop&crop=face".to_string(),
                    status: "Online".to_string(),
                    projects: 5
                }
                crate::components::TeamMemberCard {
                    name: "Bob Smith".to_string(),
                    role: "Senior Developer".to_string(),
                    email: "bob@company.com".to_string(),
                    avatar_url: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=80&h=80&fit=crop&crop=face".to_string(),
                    status: "Away".to_string(),
                    projects: 3
                }
                crate::components::TeamMemberCard {
                    name: "Carol Davis".to_string(),
                    role: "UX Designer".to_string(),
                    email: "carol@company.com".to_string(),
                    avatar_url: "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=80&h=80&fit=crop&crop=face".to_string(),
                    status: "Online".to_string(),
                    projects: 4
                }
                crate::components::TeamMemberCard {
                    name: "David Wilson".to_string(),
                    role: "Frontend Developer".to_string(),
                    email: "david@company.com".to_string(),
                    avatar_url: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=80&h=80&fit=crop&crop=face".to_string(),
                    status: "Offline".to_string(),
                    projects: 2
                }
            }
        }
    }
}
