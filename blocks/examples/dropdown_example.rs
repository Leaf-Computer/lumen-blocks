use dioxus::prelude::*;
use dioxus_blocks::components::dropdown::{
    Dropdown, DropdownTrigger, DropdownContent, DropdownItem,
    DropdownSize, DropdownLabel, DropdownSeparator, 
    DropdownCheckboxItem, DropdownRadioGroup, DropdownRadioItem
};
use dioxus_blocks::components::button::{Button, ButtonVariant};
use log;
use lucide_dioxus::{Settings, User, LogOut, Mail, MessageSquare, Plus, CreditCard, Share2};

fn main() {
    // Initialize logger for debug builds
    #[cfg(debug_assertions)]
    {
        // This simple configuration will print to stderr
        // For a real app, consider using env_logger or another implementation
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        
        log::info!("Logger initialized in debug mode");
    }
    
    dioxus::launch(DropdownExample);
}

#[component]
pub fn DropdownExample() -> Element {
    rsx! {
        div { class: "dropdown-example p-5 flex flex-col gap-5",
            // Basic Dropdown
            div {
                h3 { "Basic Dropdown" }
                div { class: "flex flex-wrap gap-6 items-start",
                    // Simple dropdown
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "Basic Dropdown"
                            }
                        }
                        
                        DropdownContent {
                            DropdownItem {
                                value: use_signal(|| "item1".to_string()),
                                index: use_signal(|| 0),
                                on_select: move |value| {
                                    log::info!("Selected: {}", value);
                                },
                                "Profile"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item2".to_string()),
                                index: use_signal(|| 1),
                                on_select: move |value| {
                                    log::info!("Selected: {}", value);
                                },
                                "Settings"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item3".to_string()),
                                index: use_signal(|| 2),
                                on_select: move |value| {
                                    log::info!("Selected: {}", value);
                                },
                                "Logout"
                            }
                        }
                    }
                    
                    // Dropdown with label and separator
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "With Label & Separator"
                            }
                        }
                        
                        DropdownContent {
                            DropdownLabel {
                                "Account"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item1".to_string()),
                                index: use_signal(|| 0),
                                "Profile"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item2".to_string()),
                                index: use_signal(|| 1),
                                "Settings"
                            }
                            
                            DropdownSeparator {}
                            
                            DropdownLabel {
                                "Actions"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item3".to_string()),
                                index: use_signal(|| 2),
                                "Logout"
                            }
                        }
                    }
                }
            }
            
            // Dropdown states
            div {
                h3 { "Dropdown States" }
                div { class: "flex flex-wrap gap-6 items-start",
                    // Disabled dropdown
                    Dropdown {
                        disabled: use_signal(|| true),
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                disabled: use_signal(|| true),
                                "Disabled Dropdown"
                            }
                        }
                        
                        DropdownContent {
                            DropdownItem {
                                value: use_signal(|| "item1".to_string()),
                                index: use_signal(|| 0),
                                "Item 1"
                            }
                        }
                    }
                    
                    // Dropdown with disabled item
                    Dropdown {
                        DropdownTrigger {
                            "With Disabled Item"
                        }
                        
                        DropdownContent {
                            DropdownItem {
                                value: use_signal(|| "item1".to_string()),
                                index: use_signal(|| 0),
                                "Normal Item"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item2".to_string()),
                                index: use_signal(|| 1),
                                disabled: use_signal(|| true),
                                "Disabled Item"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item3".to_string()),
                                index: use_signal(|| 2),
                                "Normal Item"
                            }
                        }
                    }
                    
                    // Destructive item
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "With Destructive Item"
                            }
                        }
                        
                        DropdownContent {
                            DropdownItem {
                                value: use_signal(|| "item1".to_string()),
                                index: use_signal(|| 0),
                                "Normal Item"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item2".to_string()),
                                index: use_signal(|| 1),
                                destructive: use_signal(|| true),
                                "Destructive Action"
                            }
                        }
                    }
                }
            }
            
            // Dropdown with icons
            div {
                h3 { "Dropdown with Icons" }
                div { class: "flex flex-wrap gap-6 items-start",
                    Dropdown {
                        DropdownTrigger {
                            "User Menu"
                        }
                        
                        DropdownContent {
                            DropdownItem {
                                value: use_signal(|| "profile".to_string()),
                                index: use_signal(|| 0),
                                icon: rsx! { User { size: 16, color: "currentColor" } },
                                "Profile"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "settings".to_string()),
                                index: use_signal(|| 1),
                                icon: rsx! { Settings { size: 16, color: "currentColor" } },
                                "Settings"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "logout".to_string()),
                                index: use_signal(|| 2),
                                icon: rsx! { LogOut { size: 16, color: "currentColor" } },
                                destructive: use_signal(|| true),
                                "Logout"
                            }
                        }
                    }
                    
                    Dropdown {
                        DropdownTrigger {
                            "Actions"
                        }
                        
                        DropdownContent {
                            DropdownItem {
                                value: use_signal(|| "new".to_string()),
                                index: use_signal(|| 0),
                                icon: rsx! { Plus { size: 16, color: "currentColor" } },
                                "New Item"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "share".to_string()),
                                index: use_signal(|| 1),
                                icon: rsx! { Share2 { size: 16, color: "currentColor" } },
                                "Share"
                            }
                        }
                    }
                }
            }
            
            // Dropdown alignment
            div {
                h3 { "Dropdown Alignment" }
                div { class: "flex flex-wrap gap-6 items-start",
                    // Left aligned (default)
                    Dropdown {
                        DropdownTrigger {
                            "Left Aligned"
                        }
                        
                        DropdownContent {
                            align: "start".to_string(),
                            
                            DropdownItem {
                                value: use_signal(|| "item1".to_string()),
                                index: use_signal(|| 0),
                                "Item 1"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item2".to_string()),
                                index: use_signal(|| 1),
                                "Item 2"
                            }
                        }
                    }
                    
                    // Center aligned
                    // Dropdown with different alignment
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "Center Aligned"
                            }
                        }
                        
                        DropdownContent {
                            align: "center".to_string(),
                            
                            DropdownItem {
                                value: use_signal(|| "item1".to_string()),
                                index: use_signal(|| 0),
                                "Item 1"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item2".to_string()),
                                index: use_signal(|| 1),
                                "Item 2"
                            }
                        }
                    }
                    
                    // Right aligned
                    // Dropdown with different alignment
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "Right Aligned"
                            }
                        }
                        
                        DropdownContent {
                            align: "end".to_string(),
                            
                            DropdownItem {
                                value: use_signal(|| "item1".to_string()),
                                index: use_signal(|| 0),
                                "Item 1"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "item2".to_string()),
                                index: use_signal(|| 1),
                                "Item 2"
                            }
                        }
                    }
                }
            }
            
            // Checkbox and Radio Examples
            div {
                h3 { "Checkbox and Radio Examples" }
                div { class: "flex flex-wrap gap-6 items-start",
                    // Checkbox Example
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "Settings"
                            }
                        }
                        
                        DropdownContent {
                            width: "w-56".to_string(),
                            
                            DropdownLabel {
                                "Appearance"
                            }
                            
                            {
                                let mut dark_mode = use_signal(|| true);
                                let mut compact_mode = use_signal(|| false);
                                
                                rsx! {
                                    DropdownCheckboxItem {
                                        value: use_signal(|| "dark_mode".to_string()),
                                        index: use_signal(|| 0),
                                        checked: dark_mode,
                                        on_change: move |checked| {
                                            dark_mode.set(checked);
                                        },
                                        "Dark Mode"
                                    }

                                    DropdownCheckboxItem {
                                        value: use_signal(|| "compact_mode".to_string()),
                                        index: use_signal(|| 1),
                                        checked: compact_mode,
                                        on_change: move |checked| {
                                            compact_mode.set(checked);
                                        },
                                        "Compact Mode"
                                    }
                                }
                            }
                        }
                    }
                    
                    // Radio Example
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "Theme"
                            }
                        }
                        
                        DropdownContent {
                            width: "w-56".to_string(),
                            
                            DropdownLabel {
                                "Color Theme"
                            }
                            
                            {
                                let mut theme = use_signal(|| "system".to_string());
                                
                                rsx! {
                                    DropdownRadioGroup {
                                        value: theme,
                                        on_value_change: move |value: String| {
                                            theme.set(value.clone());
                                            log::info!("Theme changed to: {}", value);
                                        },
                                        
                                        DropdownRadioItem {
                                            value: use_signal(|| "light".to_string()),
                                            index: use_signal(|| 0),
                                            "Light"
                                        }
                                    
                                        DropdownRadioItem {
                                            value: use_signal(|| "dark".to_string()),
                                            index: use_signal(|| 1),
                                            "Dark"
                                        }
                                        
                                        DropdownRadioItem {
                                            value: use_signal(|| "system".to_string()),
                                            index: use_signal(|| 2),
                                            "System"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Complex example
            div {
                h3 { "Complex Dropdown Example" }
                div { class: "flex flex-wrap gap-6 items-start",
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "Account & Settings"
                            }
                        }
                        
                        DropdownContent {
                            width: "w-64".to_string(),
                            
                            DropdownLabel {
                                "My Account"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "account".to_string()),
                                index: use_signal(|| 0),
                                icon: rsx! { User { size: 16, color: "currentColor" } },
                                "Profile"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "messages".to_string()),
                                index: use_signal(|| 1),
                                icon: rsx! { MessageSquare { size: 16, color: "currentColor" } },
                                "Messages"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "mail".to_string()),
                                index: use_signal(|| 2),
                                icon: rsx! { Mail { size: 16, color: "currentColor" } },
                                "Mail"
                            }
                            
                            DropdownSeparator {}
                            
                            DropdownLabel {
                                "Settings"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "billing".to_string()),
                                index: use_signal(|| 3),
                                icon: rsx! { CreditCard { size: 16, color: "currentColor" } },
                                "Billing"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "settings".to_string()),
                                index: use_signal(|| 4),
                                icon: rsx! { Settings { size: 16, color: "currentColor" } },
                                "Preferences"
                            }
                            
                            DropdownSeparator {}
                            
                            DropdownItem {
                                value: use_signal(|| "logout".to_string()),
                                index: use_signal(|| 5),
                                icon: rsx! { LogOut { size: 16, color: "currentColor" } },
                                destructive: use_signal(|| true),
                                "Logout"
                            }
                        }
                    }
                }
            }
        }
    }
}
