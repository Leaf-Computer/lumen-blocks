use dioxus::prelude::*;
use laminar_blocks::components::{
    button::{Button, ButtonVariant},
    input::{Input, InputSize},
    checkbox::{Checkbox},
    switch::Switch,
    label::Label,
};
use lucide_dioxus::{Mail, Lock, User, Calendar, Check, ArrowRight};
use dioxus_time::use_timeout;

#[component]
pub fn FormElementsDemo() -> Element {
    let mut form_submitted = use_signal(|| false);
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut name = use_signal(|| String::new());
    let mut remember_me = use_signal(|| false);
    let mut notifications = use_signal(|| true);
    let mut marketing = use_signal(|| false);
    
    let mut submit_form = move |_| {
        form_submitted.set(true);
        
        // Reset after 3 seconds
        let _timeout = use_timeout(std::time::Duration::from_millis(3000), move |_: String| {
            form_submitted.set(false);
        });
    };
    
    let reset_form = move |_| {
        email.set(String::new());
        password.set(String::new());
        name.set(String::new());
        remember_me.set(false);
        marketing.set(false);
        notifications.set(true);
        form_submitted.set(false);
    };

    rsx! {
        div { class: "p-6 bg-card rounded-lg shadow-sm border border-border",
            h3 { class: "text-xl font-semibold mb-4", "Form Elements" }
            p { class: "text-muted-foreground mb-6", 
                "Form elements allow users to input data with various controls."
            }
            
            // Success message when form is submitted
            if form_submitted() {
                div { class: "mb-6 p-4 bg-green-50 text-green-700 rounded-lg border border-green-200 flex items-center",
                    Check { class: "w-5 h-5 mr-2" }
                    "Form submitted successfully!"
                }
            }
            
            form { class: "space-y-6", onsubmit: move |e| { e.stop_propagation(); submit_form(e); },
                // Name input
                div { class: "space-y-2",
                    Label { for_id: "name".to_string(), "Full Name" }
                    Input {
                        id: Some("name".to_string()),
                        value: name,
                        on_change: move |evt: FormEvent| name.set(evt.value().clone()),
                        placeholder: use_signal(|| "John Doe".to_string()),
                        icon_left: rsx! { User { class: "w-4 h-4 text-muted-foreground" } },
                    }
                }
                
                // Email input
                div { class: "space-y-2",
                    Label { for_id: "email".to_string(), "Email Address" }
                    Input {
                        id: Some("email".to_string()),
                        value: email,
                        on_change: move |evt: FormEvent| email.set(evt.value().clone()),
                        placeholder: use_signal(|| "you@example.com".to_string()),
                        icon_left: rsx! { Mail { class: "w-4 h-4 text-muted-foreground" } },
                        required: true,
                        input_type: "email".to_string(),
                    }
                    p { class: "text-xs text-muted-foreground", "We'll never share your email with anyone else." }
                }
                
                // Password input
                div { class: "space-y-2",
                    Label { for_id: "password".to_string(), "Password" }
                    Input {
                        id: Some("password".to_string()),
                        value: password,
                        on_change: move |evt: FormEvent| password.set(evt.value()),
                        placeholder: use_signal(|| "Enter your password".to_string()),
                        icon_left: rsx! { Lock { class: "w-4 h-4 text-muted-foreground" } },
                        input_type: "password".to_string(),
                        required: true,
                    }
                }
                
                // Disabled input example
                div { class: "space-y-2",
                    Label { for_id: "disabled".to_string(), "Disabled Input" }
                    Input {
                        id: Some("disabled".to_string()),
                        value: use_signal(|| "This input is disabled".to_string()),
                        icon_left: rsx! { Calendar { class: "w-4 h-4 text-muted-foreground" } },
                        disabled: true,
                    }
                }
                
                // Input sizes
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    div { class: "space-y-2",
                        Label { "Small Input" }
                        Input {
                            placeholder: use_signal(|| "Small".to_string()),
                            size: use_signal(|| InputSize::Small),
                        }
                    }
                    div { class: "space-y-2",
                        Label { "Default Input" }
                        Input {
                            placeholder: use_signal(|| "Default".to_string()),
                        }
                    }
                    div { class: "space-y-2",
                        Label { "Large Input" }
                        Input {
                            placeholder: use_signal(|| "Large".to_string()),
                            size: use_signal(|| InputSize::Large),
                        }
                    }
                }
                
                // Checkbox and Switch
                div { class: "space-y-4 pt-2",
                    h4 { class: "font-medium", "Toggle Controls" }
                    
                    div { class: "flex items-center space-x-2",
                        Checkbox {
                            id: Some("remember-me".to_string()),
                            checked: remember_me,
                            on_checked_change: move |v| remember_me.set(v),
                        }
                        Label { 
                            for_id: "remember-me".to_string(),
                            class: Some("cursor-pointer".to_string()), 
                            "Remember me" 
                        }
                    }
                    
                    div { class: "flex items-center space-x-2",
                        Switch {
                            id: Some("notifications".to_string()),
                            checked: notifications,
                            on_checked_change: move |v| notifications.set(v),
                        }
                        Label { 
                            for_id: "notifications".to_string(), 
                            class: Some("cursor-pointer".to_string()),
                            "Enable notifications" 
                        }
                    }
                    
                    div { class: "flex items-center space-x-2",
                        Switch {
                            id: Some("marketing".to_string()),
                            checked: marketing,
                            on_checked_change: move |v| marketing.set(v),
                        }
                        Label { 
                            for_id: "marketing".to_string(), 
                            class: Some("cursor-pointer".to_string()),
                            "Receive marketing emails" 
                        }
                    }
                }
                
                // Form buttons
                div { class: "flex gap-4 pt-4",
                    Button {
                        variant: use_signal(|| ButtonVariant::Primary),
                        icon_right: rsx! { ArrowRight { class: "w-4 h-4" } },
                        "Submit"
                    }
                    
                    Button {
                        variant: use_signal(|| ButtonVariant::Outline),
                        on_click: reset_form,
                        "Reset"
                    }
                }
            }
        }
    }
}

#[component]
pub fn FormSection() -> Element {
    rsx! {
        section { class: "mb-10",
            h2 { class: "text-2xl font-bold mb-6", "Form Elements" }
            div { class: "space-y-6",
                FormElementsDemo {}
                
                div { class: "mt-6 p-4 bg-muted rounded-lg",
                    h4 { class: "font-medium mb-2", "Usage Notes" }
                    ul { class: "list-disc pl-5 space-y-1 text-sm text-muted-foreground",
                        li { "Form elements should have proper labels for accessibility." }
                        li { "Provide validation feedback when appropriate." }
                        li { "Use consistent sizing and spacing for a polished look." }
                        li { "Include helper text for fields that need additional explanation." }
                        li { "Group related controls together for better organization." }
                    }
                }
            }
        }
    }
}
