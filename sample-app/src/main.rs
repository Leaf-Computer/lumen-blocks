use dioxus::prelude::*;
use laminar_blocks::components::{
    avatar::{Avatar, AvatarImage, AvatarFallback},
    button::{Button, ButtonVariant, ButtonSize},
    input::{Input, InputSize},
    progress::{Progress, ProgressSize, ProgressVariant},
    switch::Switch,
};
use lucide_dioxus::{Check, Info, X};

mod components;
mod pages;
use crate::components::{ProjectCard, StatsCard};
use crate::pages::{Components, Dashboard};

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/components")]
    Components {},
    #[route("/dashboard")]
    Dashboard {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            Router::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-background",
            Navbar {}
            
            div { class: "max-w-6xl mx-auto px-6 py-12",
                div { class: "text-center mb-12",
                    h1 { class: "text-4xl font-bold text-foreground mb-4", "Laminar Blocks Demo" }
                    p { class: "text-xl text-muted-foreground mb-8", 
                        "A comprehensive component library for Dioxus applications" 
                    }
                    
                    div { class: "flex justify-center gap-4",
                        Link { to: Route::Components {},
                            Button {
                                variant: use_signal(|| ButtonVariant::Primary),
                                size: use_signal(|| ButtonSize::Large),
                                "View Components"
                            }
                        }
                        Link { to: Route::Dashboard {},
                            Button {
                                variant: use_signal(|| ButtonVariant::Ghost),
                                size: use_signal(|| ButtonSize::Large),
                                "Dashboard Demo"
                            }
                        }
                    }
                }
                
                // Feature Cards
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 mb-12",
                    FeatureCard {
                        title: "Rich Components".to_string(),
                        description: "Comprehensive set of UI components built for modern web applications".to_string(),
                        icon: rsx! { Check { class: "w-8 h-8 text-primary" } }
                    }
                    FeatureCard {
                        title: "Tailwind Styled".to_string(),
                        description: "Beautifully designed with Tailwind CSS and dark mode support".to_string(),
                        icon: rsx! { Info { class: "w-8 h-8 text-primary" } }
                    }
                    FeatureCard {
                        title: "Accessible".to_string(),
                        description: "Built with accessibility in mind, following ARIA best practices".to_string(),
                        icon: rsx! { X { class: "w-8 h-8 text-primary" } }
                    }
                }
                
                // Quick Component Preview
                div { class: "bg-card rounded-lg border border-border p-8",
                    h2 { class: "text-2xl font-semibold text-foreground mb-6", "Quick Preview" }
                    
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                        div { class: "space-y-4",
                            h3 { class: "font-medium text-foreground mb-3", "Buttons" }
                            div { class: "flex flex-wrap gap-2",
                                Button {
                                    variant: use_signal(|| ButtonVariant::Primary),
                                    "Primary"
                                }
                                Button {
                                    variant: use_signal(|| ButtonVariant::Secondary),
                                    "Secondary"
                                }
                                Button {
                                    variant: use_signal(|| ButtonVariant::Outline),
                                    "Outline"
                                }
                            }
                            
                            h3 { class: "font-medium text-foreground mb-3 mt-6", "Inputs" }
                            Input {
                                placeholder: use_signal(|| "Enter your email".to_string()),
                                size: use_signal(|| InputSize::Medium),
                                icon_left: rsx! { Info { class: "w-4 h-4" } },
                            }
                            
                            h3 { class: "font-medium text-foreground mb-3 mt-6", "Progress" }
                            Progress {
                                value: Some(use_signal(|| 75.0).into()),
                                variant: ProgressVariant::Default,
                                show_percentage: true,
                            }
                        }
                        
                        div { class: "space-y-4",
                            h3 { class: "font-medium text-foreground mb-3", "Avatars" }
                            div { class: "flex gap-2",
                                Avatar {
                                    AvatarImage {
                                        src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=40&h=40&fit=crop&crop=face".to_string(),
                                        alt: "User".to_string(),
                                    }
                                    AvatarFallback { "JD" }
                                }
                                Avatar {
                                    AvatarFallback { "AB" }
                                }
                                Avatar {
                                    class: Some("w-12 h-12".to_string()),
                                    AvatarFallback { "CD" }
                                }
                            }
                            
                            h3 { class: "font-medium text-foreground mb-3 mt-6", "Switch" }
                            div { class: "flex items-center gap-3",
                                Switch {
                                    checked: use_signal(|| true),
                                }
                                span { class: "text-sm text-muted-foreground", "Enable notifications" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
// Route handler for the Components page
fn ComponentsRoute() -> Element {
    rsx! {
        pages::Components {}
    }
}

#[component]
fn DashboardRoute() -> Element {
    rsx! {
        Dashboard {}
    }
}

#[component]
fn _Dashboard() -> Element {
    let mut project_progress = use_signal(|| 65.0);
    
    rsx! {
        div { class: "min-h-screen bg-background",
            div { class: "bg-card shadow-sm border-b border-border",
                div { class: "max-w-6xl mx-auto px-6 py-4 flex items-center justify-between",
                    h1 { class: "text-xl font-bold", "Dioxus Blocks" }
                    
                    div { class: "flex items-center gap-4",
                        a { 
                            href: "/", 
                            class: "text-sm font-medium text-foreground",
                            "Home"
                        }
                        a { 
                            href: "/components", 
                            class: "text-sm font-medium text-muted-foreground hover:text-foreground transition-colors",
                            "Components"
                        }
                        a { 
                            href: "/dashboard", 
                            class: "text-sm font-medium text-muted-foreground hover:text-foreground transition-colors",
                            "Dashboard"
                        }
                    }
                }
            }
            
            div { class: "max-w-6xl mx-auto px-6 py-8",
                h1 { class: "text-3xl font-bold text-foreground mb-6", "Project Dashboard" }
                
                div { class: "space-y-8",
                    // Stats Cards
                    div { class: "grid grid-cols-1 md:grid-cols-4 gap-6",
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
                    
                    // Interactive Progress Demo
                    div { class: "bg-card rounded-lg border border-border p-6",
                        h3 { class: "text-lg font-semibold text-foreground mb-4", "Interactive Progress Demo" }
                        p { class: "text-muted-foreground mb-4", "Adjust the slider to see the progress components in action:" }
                        
                        div { class: "space-y-4",
                            div { class: "flex items-center gap-4",
                                label { class: "text-sm font-medium text-foreground min-w-[100px]", "Progress:" }
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
                                    value: Some(use_signal(move || project_progress()).into()),
                                    size: ProgressSize::Small,
                                    variant: ProgressVariant::Default,
                                    show_percentage: true,
                                }
                                Progress {
                                    value: Some(use_signal(move || project_progress()).into()),
                                    size: ProgressSize::Medium,
                                    variant: ProgressVariant::Success,
                                    show_percentage: true,
                                }
                                Progress {
                                    value: Some(use_signal(move || project_progress()).into()),
                                    size: ProgressSize::Large,
                                    variant: ProgressVariant::Warning,
                                    show_percentage: true,
                                }
                            }
                        }
                    }
                    
                    // Project Cards
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                        crate::components::ProjectCard {
                            name: "Website Redesign".to_string(),
                            progress: 75.0,
                            status: "In Progress".to_string(),
                            team_size: 5,
                            due_date: "Dec 31, 2024".to_string(),
                        }
                        crate::components::ProjectCard {
                            name: "Mobile App".to_string(),
                            progress: 45.0,
                            status: "Planning".to_string(),
                            team_size: 8,
                            due_date: "Feb 28, 2025".to_string(),
                        }
                        ProjectCard {
                            name: "API Integration".to_string(),
                            progress: 90.0,
                            status: "Review".to_string(),
                            team_size: 3,
                            due_date: "Nov 15, 2024".to_string(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav { class: "bg-card border-b border-border px-6 py-4",
            div { class: "max-w-6xl mx-auto flex items-center justify-between",
                div { class: "flex items-center gap-2",
                    div { class: "w-8 h-8 bg-primary rounded-lg flex items-center justify-center",
                        span { class: "text-primary-foreground font-bold", "L" }
                    }
                    span { class: "text-xl font-bold text-foreground", "Laminar Blocks" }
                }
                
                div { class: "flex items-center gap-6",
                    Link { 
                        to: Route::Home {},
                        class: "text-foreground hover:text-primary transition-colors",
                        "Home" 
                    }
                    Link { 
                        to: Route::Components {},
                        class: "text-foreground hover:text-primary transition-colors",
                        "Components" 
                    }
                    Link { 
                        to: Route::Dashboard {},
                        class: "text-foreground hover:text-primary transition-colors",
                        "Dashboard" 
                    }
                }
            }
        }
    }
}

#[component]
fn FeatureCard(title: String, description: String, icon: Element) -> Element {
    rsx! {
        div { class: "bg-card rounded-lg border border-border p-6 text-center",
            div { class: "flex justify-center mb-4",
                {icon}
            }
            h3 { class: "text-lg font-semibold text-foreground mb-2", "{title}" }
            p { class: "text-muted-foreground", "{description}" }
        }
    }
}

#[component]
fn ComponentSection(title: String, description: String, children: Element) -> Element {
    rsx! {
        div { class: "bg-card rounded-lg border border-border p-6",
            div { class: "mb-6",
                h2 { class: "text-xl font-semibold text-foreground mb-2", "{title}" }
                p { class: "text-muted-foreground", "{description}" }
            }
            {children}
        }
    }
}

#[component]
fn MainStatsCard(
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

// End of components
