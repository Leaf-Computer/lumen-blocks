use dioxus::prelude::*;
use laminar_blocks::components::{
    avatar::{Avatar, AvatarImage, AvatarFallback},
    button::{Button, ButtonVariant},
    input::Input,
};
use lucide_dioxus::{Check, Info, X};

#[component]
pub fn SidebarLink(
    active: bool,
    onclick: EventHandler<MouseEvent>,
    icon: Element,
    children: Element
) -> Element {
    let base_classes = "flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors";
    let classes = if active {
        format!("{} bg-primary text-primary-foreground", base_classes)
    } else {
        format!("{} text-muted-foreground hover:text-foreground hover:bg-muted", base_classes)
    };
    
    rsx! {
        button {
            class: classes,
            onclick: onclick,
            {icon}
            {children}
        }
    }
}

#[component]
pub fn Sidebar(current_section: Signal<String>) -> Element {
    rsx! {
        aside { class: "w-64 bg-card border-r border-border shadow-sm",
            div { class: "p-6",
                div { class: "flex items-center gap-3 mb-8",
                    div { class: "w-10 h-10 bg-primary rounded-lg flex items-center justify-center",
                        span { class: "text-primary-foreground font-bold text-lg", "PM" }
                    }
                    div {
                        h1 { class: "text-xl font-bold text-foreground", "ProjectHub" }
                        p { class: "text-sm text-muted-foreground", "Management Suite" }
                    }
                }
                
                nav { class: "space-y-2",
                    SidebarLink {
                        active: current_section() == "overview",
                        onclick: move |_| current_section.set("overview".to_string()),
                        icon: rsx! { Check { class: "w-5 h-5" } },
                        "Overview"
                    }
                    SidebarLink {
                        active: current_section() == "projects",
                        onclick: move |_| current_section.set("projects".to_string()),
                        icon: rsx! { Info { class: "w-5 h-5" } },
                        "Projects"
                    }
                    SidebarLink {
                        active: current_section() == "team",
                        onclick: move |_| current_section.set("team".to_string()),
                        icon: rsx! { X { class: "w-5 h-5" } },
                        "Team"
                    }
                    SidebarLink {
                        active: current_section() == "components",
                        onclick: move |_| current_section.set("components".to_string()),
                        icon: rsx! { Info { class: "w-5 h-5" } },
                        "Components"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Header(current_section: Signal<String>) -> Element {
    rsx! {
        header { class: "bg-card border-b border-border px-6 py-4",
            div { class: "flex items-center justify-between",
                div { class: "flex items-center gap-4",
                    h2 { class: "text-2xl font-semibold text-foreground capitalize", 
                        "{current_section()}" 
                    }
                    if current_section() == "projects" {
                        Button {
                            icon_left: rsx! { Check { class: "w-4 h-4" } },
                            "New Project"
                        }
                    }
                }
                
                div { class: "flex items-center gap-4",
                    // Search
                    div { class: "relative",
                        Input {
                            placeholder: use_signal(|| "Search...".to_string()),
                            icon_left: rsx! { Info { class: "w-4 h-4 text-muted-foreground" } },
                            class: Some("w-80".to_string()),
                        }
                    }
                    
                    // Notifications
                    Button {
                        variant: use_signal(|| ButtonVariant::Ghost),
                        is_icon_button: use_signal(|| true),
                        aria_label: Some("Notifications".to_string()),
                        Info { class: "w-5 h-5" }
                    }
                    
                    // User Avatar
                    Avatar {
                        class: Some("cursor-pointer".to_string()),
                        AvatarImage {
                            src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=32&h=32&fit=crop&crop=face".to_string(),
                            alt: "User Avatar".to_string(),
                        }
                        AvatarFallback { "JD" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AppLayout(current_section: Signal<String>, children: Element) -> Element {
    rsx! {
        div { class: "flex min-h-screen bg-background",
            Sidebar { current_section }
            
            div { class: "flex-1 flex flex-col",
                Header { current_section }
                
                main { class: "flex-1 p-6 overflow-auto",
                    {children}
                }
            }
        }
    }
}
