use dioxus::prelude::*;
use crate::components::ProjectDetailCard;
use laminar_blocks::components::button::{Button, ButtonVariant};
use lucide_dioxus::{Check, Info, X};

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "min-h-screen bg-background p-6",
            div { class: "max-w-7xl mx-auto space-y-8",
                // Header
                div { class: "flex items-center justify-between",
                    div {
                        h1 { class: "text-3xl font-bold text-foreground", "Projects" }
                        p { class: "text-muted-foreground mt-2", "Manage and track all your projects in one place" }
                    }
                    div { class: "flex gap-3",
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
                        Button {
                            variant: Signal::new(ButtonVariant::Primary),
                            icon_left: rsx! { Check { class: "w-4 h-4" } },
                            "New Project"
                        }
                    }
                }
                
                // Project Stats
                div { class: "grid grid-cols-1 md:grid-cols-4 gap-6",
                    div { class: "bg-card rounded-lg border border-border p-6",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "text-sm text-muted-foreground", "Total Projects" }
                                p { class: "text-2xl font-bold text-foreground mt-1", "24" }
                            }
                            div { class: "w-12 h-12 bg-blue-100 dark:bg-blue-900/20 rounded-lg flex items-center justify-center",
                                span { class: "text-blue-600 dark:text-blue-400 text-xl", "📊" }
                            }
                        }
                    }
                    div { class: "bg-card rounded-lg border border-border p-6",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "text-sm text-muted-foreground", "In Progress" }
                                p { class: "text-2xl font-bold text-foreground mt-1", "12" }
                            }
                            div { class: "w-12 h-12 bg-orange-100 dark:bg-orange-900/20 rounded-lg flex items-center justify-center",
                                span { class: "text-orange-600 dark:text-orange-400 text-xl", "⚡" }
                            }
                        }
                    }
                    div { class: "bg-card rounded-lg border border-border p-6",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "text-sm text-muted-foreground", "Completed" }
                                p { class: "text-2xl font-bold text-foreground mt-1", "8" }
                            }
                            div { class: "w-12 h-12 bg-green-100 dark:bg-green-900/20 rounded-lg flex items-center justify-center",
                                span { class: "text-green-600 dark:text-green-400 text-xl", "✅" }
                            }
                        }
                    }
                    div { class: "bg-card rounded-lg border border-border p-6",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "text-sm text-muted-foreground", "On Hold" }
                                p { class: "text-2xl font-bold text-foreground mt-1", "4" }
                            }
                            div { class: "w-12 h-12 bg-gray-100 dark:bg-gray-900/20 rounded-lg flex items-center justify-center",
                                span { class: "text-gray-600 dark:text-gray-400 text-xl", "⏸️" }
                            }
                        }
                    }
                }
                
                // Projects Grid
                div { class: "grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6",
                    ProjectDetailCard {
                        name: "E-commerce Platform".to_string(),
                        description: "Building a modern e-commerce solution with React and Node.js. Features include user authentication, shopping cart, payment processing, and admin dashboard.".to_string(),
                        progress: 80.0,
                        status: "In Progress".to_string(),
                        team: vec!["Alice Johnson".to_string(), "Bob Smith".to_string(), "Carol Davis".to_string()],
                        tasks_completed: 24,
                        tasks_total: 30,
                        priority: "High".to_string()
                    }
                    ProjectDetailCard {
                        name: "Marketing Website".to_string(),
                        description: "Redesigning company website with focus on conversion optimization and modern responsive design.".to_string(),
                        progress: 45.0,
                        status: "Planning".to_string(),
                        team: vec!["David Wilson".to_string(), "Eve Brown".to_string()],
                        tasks_completed: 12,
                        tasks_total: 28,
                        priority: "Medium".to_string()
                    }
                    ProjectDetailCard {
                        name: "Data Analytics Dashboard".to_string(),
                        description: "Creating comprehensive analytics dashboard for business intelligence with real-time data visualization.".to_string(),
                        progress: 95.0,
                        status: "Review".to_string(),
                        team: vec!["Frank Miller".to_string(), "Grace Lee".to_string(), "Henry Taylor".to_string()],
                        tasks_completed: 19,
                        tasks_total: 20,
                        priority: "High".to_string()
                    }
                    ProjectDetailCard {
                        name: "Mobile Application".to_string(),
                        description: "Cross-platform mobile app using React Native for iOS and Android with offline capabilities.".to_string(),
                        progress: 30.0,
                        status: "In Progress".to_string(),
                        team: vec!["Ian Parker".to_string(), "Jane Foster".to_string()],
                        tasks_completed: 8,
                        tasks_total: 25,
                        priority: "Medium".to_string()
                    }
                    ProjectDetailCard {
                        name: "API Documentation".to_string(),
                        description: "Comprehensive API documentation portal with interactive examples and integration guides.".to_string(),
                        progress: 100.0,
                        status: "Complete".to_string(),
                        team: vec!["Karen White".to_string()],
                        tasks_completed: 15,
                        tasks_total: 15,
                        priority: "Low".to_string()
                    }
                    ProjectDetailCard {
                        name: "Security Audit".to_string(),
                        description: "Complete security assessment and implementation of security best practices across all systems.".to_string(),
                        progress: 60.0,
                        status: "In Progress".to_string(),
                        team: vec!["Luke Chen".to_string(), "Maria Garcia".to_string(), "Nick Rodriguez".to_string()],
                        tasks_completed: 18,
                        tasks_total: 30,
                        priority: "High".to_string()
                    }
                }
            }
        }
    }
}
