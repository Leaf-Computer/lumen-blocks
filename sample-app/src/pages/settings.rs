use dioxus::prelude::*;
use laminar_blocks::components::{
    button::{Button, ButtonVariant, ButtonSize},
    input::{Input, InputSize},
    switch::Switch,
};
use lucide_dioxus::{Check, Info, X};

#[component]
pub fn Settings() -> Element {
    let mut auto_save = use_signal(|| true);
    let mut email_notifications = use_signal(|| false);
    let mut push_notifications = use_signal(|| true);
    let mut marketing_emails = use_signal(|| false);
    let mut two_factor_auth = use_signal(|| true);
    let mut dark_mode = use_signal(|| false);
    let mut compact_mode = use_signal(|| false);
    
    let mut first_name = use_signal(|| "John".to_string());
    let mut last_name = use_signal(|| "Doe".to_string());
    let mut email = use_signal(|| "john.doe@example.com".to_string());
    let mut phone = use_signal(|| "+1 (555) 123-4567".to_string());
    let mut company = use_signal(|| "Acme Corp".to_string());
    let mut job_title = use_signal(|| "Senior Developer".to_string());
    let mut location = use_signal(|| "San Francisco, CA".to_string());
    let mut bio = use_signal(|| "Passionate developer with 8+ years of experience building web applications.".to_string());
    
    rsx! {
        div { class: "min-h-screen bg-background p-6",
            div { class: "max-w-4xl mx-auto space-y-8",
                // Header
                div {
                    h1 { class: "text-3xl font-bold text-foreground", "Settings" }
                    p { class: "text-muted-foreground mt-2", "Manage your account and application preferences" }
                }
                
                // Profile Settings
                div { class: "bg-card rounded-lg border border-border p-6",
                    h2 { class: "text-xl font-semibold text-foreground mb-6", "Profile Information" }
                    div { class: "space-y-6",
                        // Profile Picture Section
                        div { class: "flex items-center gap-6",
                            div { class: "w-20 h-20 bg-primary rounded-full flex items-center justify-center text-primary-foreground text-2xl font-bold",
                                "JD"
                            }
                            div { class: "flex-1",
                                h3 { class: "font-medium text-foreground", "Profile Picture" }
                                p { class: "text-sm text-muted-foreground mb-3", "Update your profile picture to help others recognize you" }
                                div { class: "flex gap-2",
                                    Button {
                                        variant: Signal::new(ButtonVariant::Outline),
                                        size: Signal::new(ButtonSize::Small),
                                        "Change Photo"
                                    }
                                    Button {
                                        variant: Signal::new(ButtonVariant::Ghost),
                                        size: Signal::new(ButtonSize::Small),
                                        "Remove"
                                    }
                                }
                            }
                        }
                        
                        // Basic Information
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div {
                                label { class: "block text-sm font-medium text-foreground mb-2", "First Name" }
                                Input {
                                    value: first_name,
                                    placeholder: Signal::new("Enter first name".to_string()),
                                    on_change: Some(Callback::new(move |e: Event<FormData>| {
                                        first_name.set(e.value());
                                    })),
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-foreground mb-2", "Last Name" }
                                Input {
                                    value: last_name,
                                    placeholder: Signal::new("Enter last name".to_string()),
                                    on_change: Some(Callback::new(move |e: Event<FormData>| {
                                        last_name.set(e.value());
                                    })),
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-foreground mb-2", "Email" }
                                Input {
                                    input_type: "email".to_string(),
                                    value: email,
                                    placeholder: Signal::new("Enter email address".to_string()),
                                    icon_left: rsx! { Info { class: "w-4 h-4" } },
                                    on_change: Some(Callback::new(move |e: Event<FormData>| {
                                        email.set(e.value());
                                    })),
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-foreground mb-2", "Phone" }
                                Input {
                                    input_type: "tel".to_string(),
                                    value: phone,
                                    placeholder: Signal::new("Enter phone number".to_string()),
                                    icon_left: rsx! { Check { class: "w-4 h-4" } },
                                    on_change: Some(Callback::new(move |e: Event<FormData>| {
                                        phone.set(e.value());
                                    })),
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-foreground mb-2", "Company" }
                                Input {
                                    value: company,
                                    placeholder: Signal::new("Enter company name".to_string()),
                                    on_change: Some(Callback::new(move |e: Event<FormData>| {
                                        company.set(e.value());
                                    })),
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-foreground mb-2", "Job Title" }
                                Input {
                                    value: job_title,
                                    placeholder: Signal::new("Enter job title".to_string()),
                                    on_change: Some(Callback::new(move |e: Event<FormData>| {
                                        job_title.set(e.value());
                                    })),
                                }
                            }
                        }
                        
                        div {
                            label { class: "block text-sm font-medium text-foreground mb-2", "Location" }
                            Input {
                                value: location,
                                placeholder: Signal::new("Enter your location".to_string()),
                                icon_left: rsx! { Info { class: "w-4 h-4" } },
                                on_change: Some(Callback::new(move |e: Event<FormData>| {
                                    location.set(e.value());
                                })),
                            }
                        }
                        
                        div {
                            label { class: "block text-sm font-medium text-foreground mb-2", "Bio" }
                            textarea {
                                class: "w-full min-h-[100px] px-3 py-2 text-sm border border-input rounded-md bg-background text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 resize-vertical",
                                placeholder: "Tell us about yourself...",
                                value: bio(),
                                oninput: move |e| bio.set(e.value())
                            }
                        }
                    }
                }
                
                // Preferences
                div { class: "bg-card rounded-lg border border-border p-6",
                    h2 { class: "text-xl font-semibold text-foreground mb-6", "Preferences" }
                    div { class: "space-y-6",
                        div { class: "flex items-center justify-between",
                            div {
                                h3 { class: "font-medium text-foreground", "Auto-save" }
                                p { class: "text-sm text-muted-foreground", "Automatically save your work as you type" }
                            }
                            Switch {
                                checked: auto_save,
                                on_checked_change: Some(EventHandler::new(move |checked| {
                                    auto_save.set(checked);
                                })),
                            }
                        }
                        
                        div { class: "flex items-center justify-between",
                            div {
                                h3 { class: "font-medium text-foreground", "Dark Mode" }
                                p { class: "text-sm text-muted-foreground", "Switch to dark theme interface" }
                            }
                            Switch {
                                checked: dark_mode,
                                on_checked_change: Some(EventHandler::new(move |checked| {
                                    dark_mode.set(checked);
                                })),
                            }
                        }
                        
                        div { class: "flex items-center justify-between",
                            div {
                                h3 { class: "font-medium text-foreground", "Compact Mode" }
                                p { class: "text-sm text-muted-foreground", "Use a more compact layout to fit more content" }
                            }
                            Switch {
                                checked: compact_mode,
                                on_checked_change: Some(EventHandler::new(move |checked| {
                                    compact_mode.set(checked);
                                })),
                            }
                        }
                    }
                }
                
                // Notifications
                div { class: "bg-card rounded-lg border border-border p-6",
                    h2 { class: "text-xl font-semibold text-foreground mb-6", "Notifications" }
                    div { class: "space-y-6",
                        div { class: "flex items-center justify-between",
                            div {
                                h3 { class: "font-medium text-foreground", "Email Notifications" }
                                p { class: "text-sm text-muted-foreground", "Receive notifications about project updates via email" }
                            }
                            Switch {
                                checked: email_notifications,
                                on_checked_change: Some(EventHandler::new(move |checked| {
                                    email_notifications.set(checked);
                                })),
                            }
                        }
                        
                        div { class: "flex items-center justify-between",
                            div {
                                h3 { class: "font-medium text-foreground", "Push Notifications" }
                                p { class: "text-sm text-muted-foreground", "Receive push notifications in your browser" }
                            }
                            Switch {
                                checked: push_notifications,
                                on_checked_change: Some(EventHandler::new(move |checked| {
                                    push_notifications.set(checked);
                                })),
                            }
                        }
                        
                        div { class: "flex items-center justify-between",
                            div {
                                h3 { class: "font-medium text-foreground", "Marketing Emails" }
                                p { class: "text-sm text-muted-foreground", "Receive emails about new features and updates" }
                            }
                            Switch {
                                checked: marketing_emails,
                                on_checked_change: Some(EventHandler::new(move |checked| {
                                    marketing_emails.set(checked);
                                })),
                            }
                        }
                    }
                }
                
                // Security
                div { class: "bg-card rounded-lg border border-border p-6",
                    h2 { class: "text-xl font-semibold text-foreground mb-6", "Security" }
                    div { class: "space-y-6",
                        div { class: "flex items-center justify-between",
                            div {
                                h3 { class: "font-medium text-foreground", "Two-Factor Authentication" }
                                p { class: "text-sm text-muted-foreground", "Add an extra layer of security to your account" }
                            }
                            Switch {
                                checked: two_factor_auth,
                                on_checked_change: Some(EventHandler::new(move |checked| {
                                    two_factor_auth.set(checked);
                                })),
                            }
                        }
                        
                        div { class: "space-y-3",
                            h3 { class: "font-medium text-foreground", "Change Password" }
                            p { class: "text-sm text-muted-foreground mb-3", "Update your password to keep your account secure" }
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div {
                                    label { class: "block text-sm font-medium text-foreground mb-1", "Current Password" }
                                    Input {
                                        input_type: "password".to_string(),
                                        placeholder: Signal::new("Enter current password".to_string()),
                                        size: Signal::new(InputSize::Medium),
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-foreground mb-1", "New Password" }
                                    Input {
                                        input_type: "password".to_string(),
                                        placeholder: Signal::new("Enter new password".to_string()),
                                        size: Signal::new(InputSize::Medium),
                                    }
                                }
                            }
                            Button {
                                variant: Signal::new(ButtonVariant::Outline),
                                size: Signal::new(ButtonSize::Small),
                                "Update Password"
                            }
                        }
                    }
                }
                
                // Danger Zone
                div { class: "bg-card border border-destructive/20 rounded-lg p-6",
                    h2 { class: "text-xl font-semibold text-destructive mb-6", "Danger Zone" }
                    div { class: "space-y-4",
                        div {
                            h3 { class: "font-medium text-foreground", "Delete Account" }
                            p { class: "text-sm text-muted-foreground mb-3", "Permanently delete your account and all associated data. This action cannot be undone." }
                            Button {
                                variant: Signal::new(ButtonVariant::Destructive),
                                size: Signal::new(ButtonSize::Small),
                                "Delete Account"
                            }
                        }
                    }
                }
                
                // Action Buttons
                div { class: "flex justify-end gap-3 pt-6 border-t border-border",
                    Button {
                        variant: Signal::new(ButtonVariant::Outline),
                        icon_left: rsx! { X { class: "w-4 h-4" } },
                        "Cancel"
                    }
                    Button {
                        variant: Signal::new(ButtonVariant::Primary),
                        icon_left: rsx! { Check { class: "w-4 h-4" } },
                        "Save Changes"
                    }
                }
            }
        }
    }
}
