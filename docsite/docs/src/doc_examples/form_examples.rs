#![allow(non_snake_case)]


pub use basic::BasicInputExample;
pub use variants::InputVariantsExample;
pub use sizes::InputSizesExample;
pub use icons::InputWithIconsExample;
pub use states::InputStatesExample;
pub use labels::LabelExample;
pub use label_sizes::LabelSizesExample;
pub use required::RequiredFieldExample;
pub use validation::FormValidationExample;
pub use complete::CompleteFormExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::input::Input;
    
    #[component]
    pub fn BasicInputExample() -> Element {
        let mut value = use_signal(|| String::new());
        
        rsx! {
            div { class: "w-full max-w-md",
                Input {
                    id: Some("basic-input".to_string()),
                    input_type: "text".to_string(),
                    placeholder: "Enter text".to_string(),
                    full_width: true,
                    value: value,
                    on_change: move |evt: FormEvent| value.set(evt.value().clone())
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod variants {
    // ANCHOR: variants
    use dioxus::prelude::*;
    use laminar_blocks::components::input::{Input, InputVariant};
    use lucide_dioxus::CircleAlert;
    
    #[component]
    pub fn InputVariantsExample() -> Element {
        rsx! {
            div { class: "w-full max-w-md space-y-4",
                div {
                    Input {
                        id: Some("default-input".to_string()),
                        input_type: "text".to_string(),
                        placeholder: "Default input".to_string(),
                        full_width: true,
                        variant: InputVariant::Default
                    }
                }
                
                div {
                    Input {
                        id: Some("error-input".to_string()),
                        input_type: "text".to_string(),
                        placeholder: "Error input".to_string(),
                        full_width: true,
                        variant: InputVariant::Error
                    }
                    
                    div {
                        class: "text-destructive text-xs flex items-center mt-1",
                        CircleAlert { size: 14, class: "mr-1 text-destructive" }
                        "This field has an error"
                    }
                }
            }
        }
    }
    // ANCHOR_END: variants
}

pub mod sizes {
    // ANCHOR: sizes
    use dioxus::prelude::*;
    use laminar_blocks::components::input::{Input, InputSize};
    
    #[component]
    pub fn InputSizesExample() -> Element {
        rsx! {
            div { class: "w-full max-w-md space-y-4",
                Input {
                    size: InputSize::Small,
                    placeholder: "Small input".to_string(),
                    full_width: true
                }
                
                Input {
                    size: InputSize::Medium,
                    placeholder: "Medium input".to_string(),
                    full_width: true
                }
                
                Input {
                    size: InputSize::Large,
                    placeholder: "Large input".to_string(),
                    full_width: true
                }
            }
        }
    }
    // ANCHOR_END: sizes
}

pub mod icons {
    // ANCHOR: icons
    use dioxus::prelude::*;
    use laminar_blocks::components::input::{Input};
    use lucide_dioxus::{Mail, Search, User};
    
    #[component]
    pub fn InputWithIconsExample() -> Element {
        rsx! {
            div { class: "w-full max-w-md space-y-4",
                Input {
                    placeholder: "Email address".to_string(),
                    full_width: true,
                    icon_left: rsx! { Mail { size: 18, class: "text-foreground" } }
                }
                
                Input {
                    placeholder: "Search...".to_string(),
                    full_width: true,
                    icon_left: rsx! { Search { size: 18, class: "text-foreground" } }
                }
                
                Input {
                    placeholder: "Username".to_string(),
                    full_width: true,
                    icon_left: rsx! { User { size: 18, class: "text-foreground" } }
                }
            }
        }
    }
    // ANCHOR_END: icons
}

pub mod states {
    // ANCHOR: states
    use dioxus::prelude::*;
    use laminar_blocks::components::input::{Input};
    
    #[component]
    pub fn InputStatesExample() -> Element {
        rsx! {
            div { class: "w-full max-w-md space-y-4",
                Input {
                    placeholder: "Disabled input".to_string(),
                    full_width: true,
                    disabled: true
                }
                
                Input {
                    value: "Read-only value".to_string(),
                    full_width: true,
                    readonly: true
                }
            }
        }
    }
    // ANCHOR_END: states
}

pub mod labels {
    // ANCHOR: labels
    use dioxus::prelude::*;
    use laminar_blocks::components::label::{Label};
    use laminar_blocks::components::input::{Input};
    
    #[component]
    pub fn LabelExample() -> Element {
        rsx! {
            div { class: "w-full max-w-md space-y-1",
                Label {
                    for_id: Some("labeled-input".to_string()),
                    "Email Address"
                }
                
                Input {
                    id: Some("labeled-input".to_string()),
                    input_type: "email".to_string(),
                    placeholder: "Enter your email".to_string(),
                    full_width: true
                }
            }
        }
    }
    // ANCHOR_END: labels
}

pub mod label_sizes {
    // ANCHOR: label_sizes
    use dioxus::prelude::*;
    use laminar_blocks::components::label::{Label, LabelSize};
    use laminar_blocks::components::input::{Input, InputSize};
    
    #[component]
    pub fn LabelSizesExample() -> Element {
        rsx! {
            div { class: "w-full max-w-md space-y-4",
                div { class: "space-y-1",
                    Label {
                        size: LabelSize::Small,
                        for_id: Some("small-input".to_string()),
                        "Small Label"
                    }
                    
                    Input {
                        id: Some("small-input".to_string()),
                        size: InputSize::Small,
                        full_width: true
                    }
                }
                
                div { class: "space-y-1",
                    Label {
                        size: LabelSize::Medium,
                        for_id: Some("medium-input".to_string()),
                        "Medium Label"
                    }
                    
                    Input {
                        id: Some("medium-input".to_string()),
                        size: InputSize::Medium,
                        full_width: true
                    }
                }
                
                div { class: "space-y-1",
                    Label {
                        size: LabelSize::Large,
                        for_id: Some("large-input".to_string()),
                        "Large Label"
                    }
                    
                    Input {
                        id: Some("large-input".to_string()),
                        size: InputSize::Large,
                        full_width: true
                    }
                }
            }
        }
    }
    // ANCHOR_END: label_sizes
}

pub mod required {
    // ANCHOR: required
    use dioxus::prelude::*;
    use laminar_blocks::components::label::{Label};
    use laminar_blocks::components::input::{Input};
    
    #[component]
    pub fn RequiredFieldExample() -> Element {
        rsx! {
            div { class: "w-full max-w-md space-y-4",
                div { class: "space-y-1",
                    Label {
                        for_id: Some("optional-field".to_string()),
                        "Optional Field"
                    }
                    
                    Input {
                        id: Some("optional-field".to_string()),
                        full_width: true
                    }
                }
                
                div { class: "space-y-1",
                    Label {
                        for_id: Some("required-field".to_string()),
                        required: true,
                        "Required Field"
                    }
                    
                    Input {
                        id: Some("required-field".to_string()),
                        required: true,
                        full_width: true
                    }
                }
            }
        }
    }
    // ANCHOR_END: required
}

pub mod validation {
    // ANCHOR: validation
    use dioxus::prelude::*;
    use laminar_blocks::components::label::{Label};
    use laminar_blocks::components::input::{Input, InputVariant};
    use lucide_dioxus::CircleAlert;
    
    #[component]
    pub fn FormValidationExample() -> Element {
        let mut email = use_signal(|| String::new());
        let mut has_error = use_signal(|| false);
        
        let validate_email = move |evt: FormEvent| {
            email.set(evt.value().clone());
            has_error.set(!evt.value().contains('@'));
        };
        
        rsx! {
            div { class: "w-full max-w-md",
                div { class: "space-y-1",
                    Label {
                        for_id: Some("email-input".to_string()),
                        "Email Address"
                    }
                    
                    Input {
                        id: Some("email-input".to_string()),
                        input_type: "email".to_string(),
                        placeholder: "Enter your email".to_string(),
                        full_width: true,
                        variant: if has_error() { InputVariant::Error } else { InputVariant::Default },
                        value: email,
                        on_change: validate_email
                    }
                    
                    if has_error() {
                        div {
                            class: "text-destructive text-xs flex items-center mt-1",
                            CircleAlert { size: 14, class: "mr-1 text-destructive" }
                            "Please enter a valid email address"
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: validation
}

pub mod complete {
    // ANCHOR: complete
    use dioxus::prelude::*;
    use laminar_blocks::components::label::{Label};
    use laminar_blocks::components::input::{Input, InputVariant};
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use lucide_dioxus::{Mail, Lock, CircleAlert};
    
    #[component]
    pub fn CompleteFormExample() -> Element {
        let mut email = use_signal(|| String::new());
        let mut password = use_signal(|| String::new());
        let mut has_error = use_signal(|| false);
        let mut submitted = use_signal(|| false);
        
        let handle_email_change = move |evt: FormEvent| {
            email.set(evt.value().clone());
            has_error.set(false);
            submitted.set(false);
        };
        
        let handle_password_change = move |evt: FormEvent| {
            password.set(evt.value().clone());
            submitted.set(false);
        };
        
        let handle_submit = move |_| {
            if email().is_empty() || !email().contains('@') {
                has_error.set(true);
            } else if password().len() < 6 {
                has_error.set(true);
            } else {
                has_error.set(false);
                submitted.set(true);
            }
        };
        
        rsx! {
            div { class: "w-full max-w-md p-4 border rounded-md",
                h3 { class: "text-lg font-medium mb-4", "Sign In" }
                
                div { class: "space-y-4",
                    // Email field
                    div { class: "space-y-1",
                        Label {
                            for_id: Some("email".to_string()),
                            "Email Address"
                        }
                        
                        Input {
                            id: Some("email".to_string()),
                            input_type: "email".to_string(),
                            placeholder: "Enter your email".to_string(),
                            full_width: true,
                            variant: if has_error() && email().is_empty() || (email().len() > 0 && !email().contains('@')) { 
                                InputVariant::Error
                            } else { 
                                InputVariant::Default 
                            },
                            value: email,
                            on_change: handle_email_change,
                            icon_left: rsx! { Mail { size: 18, class: "text-foreground" } }
                        }
                        
                        if has_error() && email().is_empty() {
                            div {
                                class: "text-destructive text-xs flex items-center mt-1",
                                CircleAlert { size: 14, class: "mr-1 text-destructive" }
                                "Email is required"
                            }
                        } else if has_error() && email().len() > 0 && !email().contains('@') {
                            div {
                                class: "text-destructive text-xs flex items-center mt-1",
                                CircleAlert { size: 14, class: "mr-1 text-destructive" }
                                "Please enter a valid email address"
                            }
                        }
                    }
                    
                    // Password field
                    div { class: "space-y-1",
                        Label {
                            for_id: Some("password".to_string()),
                            required: true,
                            "Password"
                        }
                        
                        Input {
                            id: Some("password".to_string()),
                            input_type: "password".to_string(),
                            placeholder: "Enter your password".to_string(),
                            full_width: true,
                            required: true,
                            variant: if has_error() && (password().is_empty() || password().len() < 6) { 
                                InputVariant::Error 
                            } else { 
                                InputVariant::Default 
                            },
                            value: password,
                            on_change: handle_password_change,
                            icon_left: rsx! { Lock { size: 18, class: "text-foreground" } }
                        }
                        
                        if has_error() && password().is_empty() {
                            div {
                                class: "text-destructive text-xs flex items-center mt-1",
                                CircleAlert { size: 14, class: "mr-1 text-destructive" }
                                "Password is required"
                            }
                        } else if has_error() && password().len() < 6 {
                            div {
                                class: "text-destructive text-xs flex items-center mt-1",
                                CircleAlert { size: 14, class: "mr-1 text-destructive" }
                                "Password must be at least 6 characters"
                            }
                        }
                    }
                    
                    // Submit button
                    Button {
                        variant: ButtonVariant::Primary,
                        button_type: "submit".to_string(),
                        full_width: true,
                        on_click: handle_submit,
                        "Sign In"
                    }
                    
                    if submitted() {
                        div {
                            class: "mt-2 p-2 bg-green-50 text-green-700 text-sm rounded",
                            "Form submitted successfully!"
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: complete
}
