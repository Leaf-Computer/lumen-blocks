use dioxus::prelude::*;
use laminar_blocks::components::{
    avatar::{Avatar, AvatarImage, AvatarFallback},
    button::{Button, ButtonVariant},
};
use lucide_dioxus::{Mail, Briefcase, ExternalLink, CircleEllipsis};

#[component]
pub fn TeamMemberCard(
    name: String,
    role: String,
    email: String,
    avatar_url: String,
    status: String,
    projects: u32,
) -> Element {
    // Determine status indicator color
    let status_color = match status.to_lowercase().as_str() {
        "online" => "bg-green-500",
        "away" => "bg-yellow-500",
        "busy" => "bg-red-500",
        "offline" => "bg-gray-400",
        _ => "bg-gray-400",
    };

    rsx! {
        div { class: "bg-card rounded-lg border border-border overflow-hidden shadow-sm",
            // Header with avatar and basic info
            div { class: "p-6",
                div { class: "flex items-center",
                    // Avatar with status indicator
                    div { class: "relative mr-4",
                        Avatar {
                            class: Some("w-16 h-16".to_string()),
                            AvatarImage {
                                src: avatar_url.clone(),
                                alt: format!("{} avatar", name),
                            }
                            AvatarFallback { 
                                "{name.split_whitespace().map(|word| word.chars().next().unwrap_or(' ')).collect::<String>()}" 
                            }
                        }
                        // Status indicator
                        div { 
                            class: "absolute bottom-0 right-0 w-4 h-4 {status_color} border-2 border-background rounded-full",
                            aria_label: "{status} status"
                        }
                    }
                    
                    // Name, role and status
                    div { class: "flex-1",
                        h3 { class: "text-lg font-semibold text-foreground", "{name}" }
                        p { class: "text-sm text-muted-foreground", "{role}" }
                        span { class: "text-xs text-muted-foreground", "{status}" }
                    }
                    
                    // Menu button
                    Button {
                        variant: use_signal(|| ButtonVariant::Ghost),
                        is_icon_button: use_signal(|| true),
                        aria_label: Some("Member options".to_string()),
                        CircleEllipsis { class: "w-4 h-4" }
                    }
                }
            }
            
            // Contact and stats
            div { class: "px-6 pb-6 space-y-4",
                // Email
                div { class: "flex items-center text-sm",
                    Mail { class: "w-4 h-4 text-muted-foreground mr-2" }
                    a { 
                        href: "mailto:{email}",
                        class: "text-primary hover:underline",
                        "{email}"
                    }
                }
                
                // Projects
                div { class: "flex items-center text-sm",
                    Briefcase { class: "w-4 h-4 text-muted-foreground mr-2" }
                    span { "Working on " }
                    span { class: "font-medium", "{projects} " }
                    span { if projects == 1 { "project" } else { "projects" } }
                }
            }
            
            // Actions
            div { class: "px-6 py-4 bg-muted/50 border-t border-border flex justify-between",
                Button {
                    variant: use_signal(|| ButtonVariant::Ghost),
                    "Message"
                }
                Button {
                    variant: use_signal(|| ButtonVariant::Outline),
                    icon_right: rsx! { ExternalLink { class: "w-4 h-4 ml-1" } },
                    "View Profile"
                }
            }
        }
    }
}
