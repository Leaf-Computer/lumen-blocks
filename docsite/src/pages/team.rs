use dioxus::prelude::*;
use crate::components::TeamMemberCard;
use laminar_blocks::components::button::{Button, ButtonVariant};
use lucide_dioxus::{Check, Info, X};

#[component]
pub fn Team() -> Element {
    rsx! {
        div { class: "min-h-screen bg-background p-6",
            div { class: "max-w-7xl mx-auto space-y-8",
                // Header
                div { class: "flex items-center justify-between",
                    div {
                        h1 { class: "text-3xl font-bold text-foreground", "Team" }
                        p { class: "text-muted-foreground mt-2", "Manage your team members and their roles" }
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
                            "Invite Member"
                        }
                    }
                }
                
                // Team Stats
                div { class: "grid grid-cols-1 md:grid-cols-4 gap-6",
                    div { class: "bg-card rounded-lg border border-border p-6",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "text-sm text-muted-foreground", "Total Members" }
                                p { class: "text-2xl font-bold text-foreground mt-1", "24" }
                            }
                            div { class: "w-12 h-12 bg-blue-100 dark:bg-blue-900/20 rounded-lg flex items-center justify-center",
                                Info { class: "w-6 h-6 text-blue-600 dark:text-blue-400" }
                            }
                        }
                    }
                    div { class: "bg-card rounded-lg border border-border p-6",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "text-sm text-muted-foreground", "Online" }
                                p { class: "text-2xl font-bold text-foreground mt-1", "18" }
                            }
                            div { class: "w-12 h-12 bg-green-100 dark:bg-green-900/20 rounded-lg flex items-center justify-center",
                                Check { class: "w-6 h-6 text-green-600 dark:text-green-400" }
                            }
                        }
                    }
                    div { class: "bg-card rounded-lg border border-border p-6",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "text-sm text-muted-foreground", "Away" }
                                p { class: "text-2xl font-bold text-foreground mt-1", "4" }
                            }
                            div { class: "w-12 h-12 bg-yellow-100 dark:bg-yellow-900/20 rounded-lg flex items-center justify-center",
                                Info { class: "w-6 h-6 text-yellow-600 dark:text-yellow-400" }
                            }
                        }
                    }
                    div { class: "bg-card rounded-lg border border-border p-6",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "text-sm text-muted-foreground", "Offline" }
                                p { class: "text-2xl font-bold text-foreground mt-1", "2" }
                            }
                            div { class: "w-12 h-12 bg-gray-100 dark:bg-gray-900/20 rounded-lg flex items-center justify-center",
                                X { class: "w-6 h-6 text-gray-600 dark:text-gray-400" }
                            }
                        }
                    }
                }
                
                // Team Members Grid
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6",
                    TeamMemberCard {
                        name: "Alice Johnson".to_string(),
                        role: "Product Manager".to_string(),
                        email: "alice@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1494790108755-2616b612b786?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Online".to_string(),
                        projects: 5
                    }
                    TeamMemberCard {
                        name: "Bob Smith".to_string(),
                        role: "Senior Developer".to_string(),
                        email: "bob@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Away".to_string(),
                        projects: 3
                    }
                    TeamMemberCard {
                        name: "Carol Davis".to_string(),
                        role: "UX Designer".to_string(),
                        email: "carol@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Online".to_string(),
                        projects: 4
                    }
                    TeamMemberCard {
                        name: "David Wilson".to_string(),
                        role: "Frontend Developer".to_string(),
                        email: "david@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Offline".to_string(),
                        projects: 2
                    }
                    TeamMemberCard {
                        name: "Eve Brown".to_string(),
                        role: "Backend Developer".to_string(),
                        email: "eve@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Online".to_string(),
                        projects: 3
                    }
                    TeamMemberCard {
                        name: "Frank Miller".to_string(),
                        role: "DevOps Engineer".to_string(),
                        email: "frank@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Online".to_string(),
                        projects: 2
                    }
                    TeamMemberCard {
                        name: "Grace Lee".to_string(),
                        role: "Data Analyst".to_string(),
                        email: "grace@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1487412720507-e7ab37603c6f?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Away".to_string(),
                        projects: 1
                    }
                    TeamMemberCard {
                        name: "Henry Taylor".to_string(),
                        role: "QA Engineer".to_string(),
                        email: "henry@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Online".to_string(),
                        projects: 4
                    }
                    TeamMemberCard {
                        name: "Ian Parker".to_string(),
                        role: "Mobile Developer".to_string(),
                        email: "ian@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Online".to_string(),
                        projects: 2
                    }
                    TeamMemberCard {
                        name: "Jane Foster".to_string(),
                        role: "UI Designer".to_string(),
                        email: "jane@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1494790108755-2616b612b786?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Online".to_string(),
                        projects: 3
                    }
                    TeamMemberCard {
                        name: "Karen White".to_string(),
                        role: "Technical Writer".to_string(),
                        email: "karen@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Away".to_string(),
                        projects: 1
                    }
                    TeamMemberCard {
                        name: "Luke Chen".to_string(),
                        role: "Security Engineer".to_string(),
                        email: "luke@company.com".to_string(),
                        avatar_url: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=80&h=80&fit=crop&crop=face".to_string(),
                        status: "Online".to_string(),
                        projects: 2
                    }
                }
            }
        }
    }
}
