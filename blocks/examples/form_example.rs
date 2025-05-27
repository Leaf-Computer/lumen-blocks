use dioxus::prelude::*;
use dioxus_blocks::components::label::{Label, LabelSize};
use dioxus_blocks::components::input::{Input, InputSize, InputVariant};
use dioxus_blocks::components::button::{Button, ButtonVariant};
use lucide_dioxus::{Mail, Lock, CircleAlert};

fn main() {
    // Initialize logger for debug builds
    #[cfg(debug_assertions)]
    {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        
        log::info!("Logger initialized in debug mode");
    }
    
    dioxus::launch(FormExample);
}

#[component]
pub fn FormExample() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut has_error = use_signal(|| false);
    
    let handle_email_change = move |evt: FormEvent| {
        email.set(evt.value().clone());
        // Reset error state when user starts typing
        has_error.set(false);
    };
    
    let handle_submit = move |_| {
        // Simple validation example
        if email().is_empty() || !email().contains('@') {
            has_error.set(true);
        } else {
            has_error.set(false);
            log::info!("Form submitted with email: {} and password: {}", email(), password());
        }
    };
    
    rsx! {
        div { class: "form-example", style: "padding: 20px; max-width: 500px; margin: 0 auto;",
            h2 { "Form Components Example" }
            
            div { style: "margin-bottom: 20px;",
                // Basic Label and Input
                div { style: "margin-bottom: 16px;",
                    Label {
                        for_id: use_signal(|| Some("email".to_string())),
                        "Email Address"
                    }
                    
                    Input {
                        id: use_signal(|| Some("email".to_string())),
                        input_type: "email".to_string(),
                        placeholder: use_signal(|| "Enter your email".to_string()),
                        full_width: use_signal(|| true),
                        variant: use_signal(|| if has_error() { InputVariant::Error } else { InputVariant::Default }),
                        value: email,
                        on_change: handle_email_change,
                        icon_left: rsx! { Mail { size: 18, class: "text-foreground" } }
                    }
                    
                    // Show error message if validation fails
                    if has_error() {
                        div {
                            class: "text-destructive text-xs flex items-center mt-1",
                            CircleAlert { size: 14, class: "mr-1 text-destructive" }
                            "Please enter a valid email address"
                        }
                    }
                }
                
                // Password field with label
                div { style: "margin-bottom: 16px;",
                    Label {
                        for_id: use_signal(|| Some("password".to_string())),
                        required: use_signal(|| true),
                        "Password"
                    }
                    
                    Input {
                        id: use_signal(|| Some("password".to_string())),
                        input_type: "password".to_string(),
                        placeholder: use_signal(|| "Enter your password".to_string()),
                        full_width: use_signal(|| true),
                        required: use_signal(|| true),
                        value: password,
                        on_change: move |evt: FormEvent| password.set(evt.value().clone()),
                        icon_left: rsx! { Lock { size: 18, class: "text-foreground" } }
                    }
                }
                
                // Different sizes and variants
                div { style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; margin-bottom: 16px;",
                    div {
                        Label {
                            size: use_signal(|| LabelSize::Small),
                            "Small Input"
                        }
                        Input {
                            size: use_signal(|| InputSize::Small),
                            placeholder: use_signal(|| "Small".to_string()),
                            full_width: use_signal(|| true)
                        }
                    }
                    
                    div {
                        Label {
                            size: use_signal(|| LabelSize::Medium),
                            "Medium Input"
                        }
                        Input {
                            size: use_signal(|| InputSize::Medium),
                            placeholder: use_signal(|| "Medium".to_string()),
                            full_width: use_signal(|| true)
                        }
                    }
                    
                    div {
                        Label {
                            size: use_signal(|| LabelSize::Large),
                            "Large Input"
                        }
                        Input {
                            size: use_signal(|| InputSize::Large),
                            placeholder: use_signal(|| "Large".to_string()),
                            full_width: use_signal(|| true)
                        }
                    }
                }
                
                // Input states example
                div { style: "margin-bottom: 16px;",
                    h3 { class: "mb-2 text-base font-medium", "Input States" }
                    
                    div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 10px;",
                        div {
                            Label {
                                for_id: use_signal(|| Some("disabled-input".to_string())),
                                "Disabled Input"
                            }
                            Input {
                                id: use_signal(|| Some("disabled-input".to_string())),
                                placeholder: use_signal(|| "This input is disabled".to_string()),
                                disabled: use_signal(|| true),
                                full_width: use_signal(|| true),
                                icon_left: rsx! { Lock { size: 18, class: "text-foreground" } }
                            }
                        }
                        
                        div {
                            Label {
                                for_id: use_signal(|| Some("readonly-input".to_string())),
                                "Read-only Input"
                            }
                            Input {
                                id: use_signal(|| Some("readonly-input".to_string())),
                                value: use_signal(|| "This value cannot be changed".to_string()),
                                readonly: use_signal(|| true),
                                full_width: use_signal(|| true)
                            }
                        }
                    }
                }
                
                // Submit button
                Button {
                    variant: use_signal(|| ButtonVariant::Primary),
                    button_type: "submit".to_string(),
                    full_width: use_signal(|| true),
                    on_click: handle_submit,
                    "Submit Form"
                }
            }
        }
    }
}
