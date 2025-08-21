#![allow(non_snake_case)]

pub use alignment::DropdownAlignmentExample;
pub use basic::BasicDropdownExample;
pub use checkbox_radio::DropdownCheckboxRadioExample;
pub use complex::ComplexDropdownExample;
pub use icons::DropdownWithIconsExample;
pub use states::DropdownStatesExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use lumen_blocks::components::button::{Button, ButtonVariant};
    use lumen_blocks::components::dropdown::{
        Dropdown, DropdownContent, DropdownItem, DropdownLabel, DropdownSeparator, DropdownTrigger,
    };

    #[component]
    pub fn BasicDropdownExample() -> Element {
        let mut selected_action = use_signal(|| String::new());

        rsx! {
            div { class: "flex flex-col gap-4",
                div { class: "flex flex-wrap gap-6 items-start",
                    // Simple dropdown
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: ButtonVariant::Outline,
                                "Basic Dropdown"
                            }
                        }

                        DropdownContent {
                            DropdownItem {
                                value: "Profile".to_string(),
                                index: 0,
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Profile"
                            }

                            DropdownItem {
                                value: "Settings".to_string(),
                                index: 1,
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Settings"
                            }

                            DropdownItem {
                                value: "Logout".to_string(),
                                index: 2,
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
                                variant: ButtonVariant::Outline,
                                "With Label & Separator"
                            }
                        }

                        DropdownContent {
                            DropdownLabel {
                                "Account"
                            }

                            DropdownItem {
                                value: "Profile".to_string(),
                                index: 0,
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Profile"
                            }

                            DropdownItem {
                                value: "Settings".to_string(),
                                index: 1,
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
                                value: "Logout".to_string(),
                                index: 2,
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
                div { class: "p-3 bg-muted rounded-md my-4",
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
    use lumen_blocks::components::button::{Button, ButtonVariant};
    use lumen_blocks::components::dropdown::{
        Dropdown, DropdownContent, DropdownItem, DropdownTrigger,
    };

    #[component]
    pub fn DropdownStatesExample() -> Element {
        rsx! {
            div { class: "flex flex-wrap gap-6 items-start",
                // Disabled dropdown
                Dropdown {
                    disabled: true,
                    DropdownTrigger {
                        Button {
                            variant: ButtonVariant::Outline,
                            disabled: true,
                            "Disabled Dropdown"
                        }
                    }

                    DropdownContent {
                        DropdownItem::<String> {
                            value: "item1".to_string(),
                            index: 0,
                            "Item 1"
                        }
                    }
                }

                // Dropdown with disabled item
                Dropdown {
                    DropdownTrigger {
                        Button {
                            variant: ButtonVariant::Outline,
                            "With Disabled Item"
                        }
                    }

                    DropdownContent {
                        DropdownItem::<String> {
                            value: "item1".to_string(),
                            index: 0,
                            "Normal Item"
                        }

                        DropdownItem::<String> {
                            value: "item2".to_string(),
                            index: 1,
                            disabled: true,
                            "Disabled Item"
                        }

                        DropdownItem::<String> {
                            value: "item3".to_string(),
                            index: 2,
                            "Normal Item"
                        }
                    }
                }

                // Destructive item
                Dropdown {
                    DropdownTrigger {
                        Button {
                            variant: ButtonVariant::Outline,
                            "With Destructive Item"
                        }
                    }

                    DropdownContent {
                        DropdownItem::<String> {
                            value: "item1".to_string(),
                            index: 0,
                            "Normal Item"
                        }

                        DropdownItem::<String> {
                            value: "item2".to_string(),
                            index: 1,
                            destructive: true,
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
    use lucide_dioxus::{LogOut, Plus, Settings, Share2, User};
    use lumen_blocks::components::button::{Button, ButtonVariant};
    use lumen_blocks::components::dropdown::{
        Dropdown, DropdownContent, DropdownItem, DropdownTrigger,
    };

    #[component]
    pub fn DropdownWithIconsExample() -> Element {
        let mut selected_action = use_signal(|| String::new());

        rsx! {
            div { class: "flex flex-col gap-4",
                div { class: "flex flex-wrap gap-6 items-start",
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: ButtonVariant::Outline,
                                "User Menu"
                            }
                        }

                        DropdownContent {
                            DropdownItem {
                                value: "Profile".to_string(),
                                index: 0,
                                icon: rsx! { User { size: 16 } },
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Profile"
                            }

                            DropdownItem {
                                value: "Settings".to_string(),
                                index: 1,
                                icon: rsx! { Settings { size: 16 } },
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "Settings"
                            }

                            DropdownItem {
                                value: "Logout".to_string(),
                                index: 2,
                                icon: rsx! { LogOut { size: 16 } },
                                destructive: true,
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
                                variant: ButtonVariant::Outline,
                                "Actions"
                            }
                        }

                        DropdownContent {
                            DropdownItem {
                                value: "New Item".to_string(),
                                index: 0,
                                icon: rsx! { Plus { size: 16 } },
                                on_select: move |value| {
                                    selected_action.set(value);
                                },
                                "New Item"
                            }

                            DropdownItem {
                                value: "Share".to_string(),
                                index: 1,
                                icon: rsx! { Share2 { size: 16 } },
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
    use lumen_blocks::components::button::{Button, ButtonVariant};
    use lumen_blocks::components::dropdown::{
        Dropdown, DropdownContent, DropdownItem, DropdownTrigger,
    };

    #[component]
    pub fn DropdownAlignmentExample() -> Element {
        let mut selected_item = use_signal(|| String::new());
        let mut selected_alignment = use_signal(|| String::new());

        rsx! {
            div { class: "flex flex-col gap-4 mb-4",
                div { class: "flex flex-wrap gap-6 items-start",
                    // Left aligned (default)
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: ButtonVariant::Outline,
                                "Left Aligned"
                            }
                        }

                        DropdownContent {
                            align: "start".to_string(),

                            DropdownItem {
                                value: "Item 1".to_string(),
                                index: 0,
                                on_select: move |value| {
                                    selected_item.set(value);
                                    selected_alignment.set("start".to_string());
                                },
                                "Item 1"
                            }

                            DropdownItem {
                                value: "Item 2".to_string(),
                                index: 1,
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
                                variant: ButtonVariant::Outline,
                                "Center Aligned"
                            }
                        }

                        DropdownContent {
                            align: "center".to_string(),

                            DropdownItem {
                                value: "Item 1".to_string(),
                                index: 0,
                                on_select: move |value| {
                                    selected_item.set(value);
                                    selected_alignment.set("center".to_string());
                                },
                                "Item 1"
                            }

                            DropdownItem {
                                value: "Item 2".to_string(),
                                index: 1,
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
                                variant: ButtonVariant::Outline,
                                "Right Aligned"
                            }
                        }

                        DropdownContent {
                            align: "end".to_string(),

                            DropdownItem {
                                value: "Item 1".to_string(),
                                index: 0,
                                on_select: move |value| {
                                    selected_item.set(value);
                                    selected_alignment.set("end".to_string());
                                },
                                "Item 1"
                            }

                            DropdownItem {
                                value: "Item 2".to_string(),
                                index: 1,
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
    use lumen_blocks::components::button::{Button, ButtonVariant};
    use lumen_blocks::components::dropdown::{
        Dropdown, DropdownCheckboxItem, DropdownContent, DropdownLabel, DropdownRadioGroup,
        DropdownRadioItem, DropdownTrigger,
    };

    #[component]
    pub fn DropdownCheckboxRadioExample() -> Element {
        let mut dark_mode = use_signal(|| true);
        let mut compact_mode = use_signal(|| false);
        let mut theme = use_signal(|| "system".to_string());
        rsx! {
            div { class: "flex flex-col gap-4",
                div { class: "flex flex-wrap gap-6 items-start",
                    // Checkbox Example
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: ButtonVariant::Outline,
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
                                        index: 0_usize,
                                        checked: dark_mode(),
                                        on_change: move |checked| {
                                            dark_mode.set(checked);
                                        },
                                        "Dark Mode"
                                    }

                                    DropdownCheckboxItem {
                                        index: 1_usize,
                                        checked: compact_mode(),
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
                                variant: ButtonVariant::Outline,
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
                                            value: "light".to_string(),
                                            index: 0,
                                            "Light"
                                        }

                                        DropdownRadioItem {
                                            value: "dark".to_string(),
                                            index: 1,
                                            "Dark"
                                        }

                                        DropdownRadioItem {
                                            value: "system".to_string(),
                                            index: 2,
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
    use lucide_dioxus::{CreditCard, LogOut, Mail, MessageSquare, Settings, User};
    use lumen_blocks::components::button::{Button, ButtonVariant};
    use lumen_blocks::components::dropdown::{
        Dropdown, DropdownContent, DropdownItem, DropdownLabel, DropdownSeparator, DropdownTrigger,
    };

    #[component]
    pub fn ComplexDropdownExample() -> Element {
        let mut selected_section = use_signal(|| String::new());
        let mut selected_action = use_signal(|| String::new());

        rsx! {
            div { class: "flex flex-col gap-4",
                div { class: "flex flex-wrap gap-6 items-start",
                    Dropdown {
                        DropdownTrigger {
                            Button {
                                variant: ButtonVariant::Outline,
                                "Account & Settings"
                            }
                        }

                        DropdownContent {
                            width: "w-64".to_string(),

                            DropdownLabel {
                                "My Account"
                            }

                            DropdownItem {
                                value: "Profile".to_string(),
                                index: 0,
                                icon: rsx! { User { size: 16 } },
                                on_select: move |value| {
                                    selected_section.set("My Account".to_string());
                                    selected_action.set(value);
                                },
                                "Profile"
                            }

                            DropdownItem {
                                value: "Messages".to_string(),
                                index: 1,
                                icon: rsx! { MessageSquare { size: 16 } },
                                on_select: move |value| {
                                    selected_section.set("My Account".to_string());
                                    selected_action.set(value);
                                },
                                "Messages"
                            }

                            DropdownItem {
                                value: "Mail".to_string(),
                                index: 2,
                                icon: rsx! { Mail { size: 16 } },
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
                                value: "Billing".to_string(),
                                index: 3,
                                icon: rsx! { CreditCard { size: 16 } },
                                on_select: move |value| {
                                    selected_section.set("Settings".to_string());
                                    selected_action.set(value);
                                },
                                "Billing"
                            }

                            DropdownItem {
                                value: "Preferences".to_string(),
                                index: 4,
                                icon: rsx! { Settings { size: 16 } },
                                on_select: move |value| {
                                    selected_section.set("Settings".to_string());
                                    selected_action.set(value);
                                },
                                "Preferences"
                            }

                            DropdownSeparator {}

                            DropdownItem {
                                value: "Logout".to_string(),
                                index: 5,
                                icon: rsx! { LogOut { size: 16 } },
                                destructive: true,
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
