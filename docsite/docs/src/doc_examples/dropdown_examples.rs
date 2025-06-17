#![allow(non_snake_case)]

use lucide_dioxus::{
    User, Settings, LogOut, Plus, Share2, 
    MessageSquare, Mail, CreditCard
};

pub use basic::BasicDropdownExample;
pub use states::DropdownStatesExample;
pub use icons::DropdownWithIconsExample;
pub use alignment::DropdownAlignmentExample;
pub use checkbox_radio::DropdownCheckboxRadioExample;
pub use complex::ComplexDropdownExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::dropdown::{
        Dropdown, DropdownTrigger, DropdownContent, 
        DropdownItem, DropdownLabel, DropdownSeparator
    };
    use laminar_blocks::components::button::{Button, ButtonVariant};
    
    #[component]
    pub fn BasicDropdownExample() -> Element {
        let mut selected_action = use_signal(|| String::new());
        
        rsx! {
            div { class: "flex flex-col gap-4 pt-12 pb-36",
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
                                value: use_signal(|| "Profile".to_string()),
                                index: use_signal(|| 0),
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Profile"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "Settings".to_string()),
                                index: use_signal(|| 1),
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Settings"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "Logout".to_string()),
                                index: use_signal(|| 2),
                                on_select: move |value| {
                                    selected_action.set(value);
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
                                value: use_signal(|| "Profile".to_string()),
                                index: use_signal(|| 0),
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Profile"
                            }
                        
                            DropdownItem {
                                value: use_signal(|| "Settings".to_string()),
                                index: use_signal(|| 1),
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Settings"
                            }
                        
                            DropdownSeparator {}
                        
                            DropdownLabel {
                                "Actions"
                            }
                        
                            DropdownItem {
                                value: use_signal(|| "Logout".to_string()),
                                index: use_signal(|| 2),
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Logout"
                            }
                        }
                    }
                }
            }
                
            if !selected_action().is_empty() {
                div { class: "p-3 bg-muted rounded-md",
                    "Selected: " strong { "{selected_action()}" }
                }
            }
        }
    }
    // ANCHOR_END: basic
}


pub mod states {
    // ANCHOR: states
    use dioxus::prelude::*;
    use laminar_blocks::components::dropdown::{
        Dropdown, DropdownTrigger, DropdownContent, DropdownItem
    };
    use laminar_blocks::components::button::{Button, ButtonVariant};
    
    #[component]
    pub fn DropdownStatesExample() -> Element {
        rsx! {
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
                        Button {
                            variant: use_signal(|| ButtonVariant::Outline),
                            "With Disabled Item"
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
    }
    // ANCHOR_END: states
}

pub mod icons {
    // ANCHOR: icons
    use dioxus::prelude::*;
    use laminar_blocks::components::dropdown::{
        Dropdown, DropdownTrigger, DropdownContent, DropdownItem
    };
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use lucide_dioxus::{User, Settings, LogOut, Plus, Share2};

    #[component]
    pub fn DropdownWithIconsExample() -> Element {
        let mut selected_action = use_signal(|| String::new());

        rsx! {
            div { class: "flex flex-col gap-4 pt-12 pb-36",
                div { class: "flex flex-wrap gap-6 items-start",
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "User Menu"
                            }
                        }

                        DropdownContent {
                            DropdownItem {
                                value: use_signal(|| "Profile".to_string()),
                                index: use_signal(|| 0),
                                icon: rsx! { User { size: 16, color: "currentColor" } },
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Profile"
                            }

                            DropdownItem {
                                value: use_signal(|| "Settings".to_string()),
                                index: use_signal(|| 1),
                                icon: rsx! { Settings { size: 16, color: "currentColor" } },
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Settings"
                            }

                            DropdownItem {
                                value: use_signal(|| "Logout".to_string()),
                                index: use_signal(|| 2),
                                icon: rsx! { LogOut { size: 16, color: "currentColor" } },
                                destructive: use_signal(|| true),
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Logout"
                            }
                        }
                    }

                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "Actions"
                            }
                        }

                        DropdownContent {
                            DropdownItem {
                                value: use_signal(|| "New Item".to_string()),
                                index: use_signal(|| 0),
                                icon: rsx! { Plus { size: 16, color: "currentColor" } },
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "New Item"
                            }

                            DropdownItem {
                                value: use_signal(|| "Share".to_string()),
                                index: use_signal(|| 1),
                                icon: rsx! { Share2 { size: 16, color: "currentColor" } },
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Share"
                            }
                        }
                    }
                }

                if !selected_action().is_empty() {
                    div { class: "p-3 bg-muted rounded-md",
                        "Selected: " strong { "{selected_action()}" }
                    }
                }
            }
        }
    }
    // ANCHOR_END: icons
}

pub mod alignment {
    // ANCHOR: alignment
    use dioxus::prelude::*;
    use laminar_blocks::components::dropdown::{
        Dropdown, DropdownTrigger, DropdownContent, DropdownItem
    };
    use laminar_blocks::components::button::{Button, ButtonVariant};
    
    #[component]
    pub fn DropdownAlignmentExample() -> Element {
        let mut selected_item = use_signal(|| String::new());
        let mut selected_alignment = use_signal(|| String::new());
        
        rsx! {
            div { class: "flex flex-col gap-4 pt-12 pb-36",
                div { class: "flex flex-wrap gap-6 items-start",
                    // Left aligned (default)
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: use_signal(|| ButtonVariant::Outline),
                                "Left Aligned"
                            }
                        }
                        
                        DropdownContent {
                            align: "start".to_string(),
                            
                            DropdownItem {
                                value: use_signal(|| "Item 1".to_string()),
                                index: use_signal(|| 0),
                                on_select: move |value| {
                                    selected_item.set(value);
                                    selected_alignment.set("start".to_string());
                                },
                                "Item 1"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "Item 2".to_string()),
                                index: use_signal(|| 1),
                                on_select: move |value| {
                                    selected_item.set(value);
                                    selected_alignment.set("start".to_string());
                                },
                                "Item 2"
                            }
                        }
                    }
                    
                    // Center aligned
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
                                value: use_signal(|| "Item 1".to_string()),
                                index: use_signal(|| 0),
                                on_select: move |value| {
                                    selected_item.set(value);
                                    selected_alignment.set("center".to_string());
                                },
                                "Item 1"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "Item 2".to_string()),
                                index: use_signal(|| 1),
                                on_select: move |value| {
                                    selected_item.set(value);
                                    selected_alignment.set("center".to_string());
                                },
                                "Item 2"
                            }
                        }
                    }
                    
                    // Right aligned
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
                                value: use_signal(|| "Item 1".to_string()),
                                index: use_signal(|| 0),
                                on_select: move |value| {
                                    selected_item.set(value);
                                    selected_alignment.set("end".to_string());
                                },
                                "Item 1"
                            }
                            
                            DropdownItem {
                                value: use_signal(|| "Item 2".to_string()),
                                index: use_signal(|| 1),
                                on_select: move |value| {
                                    selected_item.set(value);
                                    selected_alignment.set("end".to_string());
                                },
                                "Item 2"
                            }
                        }
                    }
                }
            }
            
            if !selected_item().is_empty() {
                div { class: "p-3 bg-muted rounded-md",
                    "Selected: " strong { "{selected_item()}" } " from " strong { "{selected_alignment()}" } " aligned dropdown"
                }
            }
        }
    }
    // ANCHOR_END: alignment
}

pub mod checkbox_radio {
    // ANCHOR: checkbox_radio
    use dioxus::prelude::*;
    use laminar_blocks::components::dropdown::{
        Dropdown, DropdownTrigger, DropdownContent, 
        DropdownLabel, DropdownCheckboxItem, DropdownRadioGroup, DropdownRadioItem
    };
    use laminar_blocks::components::button::{Button, ButtonVariant};
    
    #[component]
    pub fn DropdownCheckboxRadioExample() -> Element {
        let mut dark_mode = use_signal(|| true);
        let mut compact_mode = use_signal(|| false);
        let mut theme = use_signal(|| "system".to_string());
        rsx! {
            div { class: "flex flex-col gap-4 pt-12 pb-36",
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
                                rsx! {
                                    DropdownRadioGroup {
                                        value: theme,
                                        on_value_change: move |value: String| {
                                            theme.set(value.clone());
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
                    
                div { class: "p-3 bg-muted rounded-md space-y-1",
                    div { 
                        "Dark Mode: " 
                        span { 
                            class: if dark_mode() { "text-green-600" } else { "text-red-600" },
                            if dark_mode() { "Enabled" } else { "Disabled" } 
                        } 
                    }
                    div { 
                        "Compact Mode: " 
                        span { 
                            class: if compact_mode() { "text-green-600" } else { "text-red-600" },
                            if compact_mode() { "Enabled" } else { "Disabled" } 
                        } 
                    }
                    div { 
                        "Theme: " 
                        strong { "{theme()}" } 
                    }
                }
            }
        }
    }
    // ANCHOR_END: checkbox_radio
}

pub mod complex {
    // ANCHOR: complex
    use dioxus::prelude::*;
    use laminar_blocks::components::dropdown::{
        Dropdown, DropdownTrigger, DropdownContent, 
        DropdownItem, DropdownLabel, DropdownSeparator
    };
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use lucide_dioxus::{User, MessageSquare, Mail, CreditCard, Settings, LogOut};

    #[component]
    pub fn ComplexDropdownExample() -> Element {
        let mut selected_section = use_signal(|| String::new());
        let mut selected_action = use_signal(|| String::new());

        rsx! {
            div { class: "flex flex-col gap-4 pt-12 pb-36 p-20",
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
                                value: use_signal(|| "Profile".to_string()),
                                index: use_signal(|| 0),
                                icon: rsx! { User { size: 16, color: "currentColor" } },
                                on_select: move |value| {
                                    selected_section.set("My Account".to_string());
                                    selected_action.set(value);
                                },
                                "Profile"
                            }

                            DropdownItem {
                                value: use_signal(|| "Messages".to_string()),
                                index: use_signal(|| 1),
                                icon: rsx! { MessageSquare { size: 16, color: "currentColor" } },
                                on_select: move |value| {
                                    selected_section.set("My Account".to_string());
                                    selected_action.set(value);
                                },
                                "Messages"
                            }

                            DropdownItem {
                                value: use_signal(|| "Mail".to_string()),
                                index: use_signal(|| 2),
                                icon: rsx! { Mail { size: 16, color: "currentColor" } },
                                on_select: move |value| {
                                    selected_section.set("My Account".to_string());
                                    selected_action.set(value);
                                },
                                "Mail"
                            }

                            DropdownSeparator {}

                            DropdownLabel {
                                "Settings"
                            }

                            DropdownItem {
                                value: use_signal(|| "Billing".to_string()),
                                index: use_signal(|| 3),
                                icon: rsx! { CreditCard { size: 16, color: "currentColor" } },
                                on_select: move |value| {
                                    selected_section.set("Settings".to_string());
                                    selected_action.set(value);
                                },
                                "Billing"
                            }

                            DropdownItem {
                                value: use_signal(|| "Preferences".to_string()),
                                index: use_signal(|| 4),
                                icon: rsx! { Settings { size: 16, color: "currentColor" } },
                                on_select: move |value| {
                                    selected_section.set("Settings".to_string());
                                    selected_action.set(value);
                                },
                                "Preferences"
                            }

                            DropdownSeparator {}

                            DropdownItem {
                                value: use_signal(|| "Logout".to_string()),
                                index: use_signal(|| 5),
                                icon: rsx! { LogOut { size: 16, color: "currentColor" } },
                                destructive: use_signal(|| true),
                                on_select: move |value| {
                                    selected_section.set("Action".to_string());
                                    selected_action.set(value);
                                },
                                "Logout"
                            }
                        }
                    }
                }

                if !selected_action().is_empty() {
                    div { class: "p-3 bg-muted rounded-md",
                        "Selected from "
                        strong { "{selected_section()}" }
                        ": "
                        strong { "{selected_action()}" }
                    }
                }
            }
        }
    }
    // ANCHOR_END: complex
}
