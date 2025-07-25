#![allow(non_snake_case)]

pub use basic::BasicMenubarExample;
pub use disabled::DisabledMenubarExample;
pub use with_icons::MenubarWithIconsExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use lumen_blocks::components::menubar::{
        Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
    };

    #[component]
    pub fn BasicMenubarExample() -> Element {
        let mut last_action = use_signal(|| String::new());

        let file_open = move |value: String| {
            last_action.set(format!("File menu selected: {}", value));
        };

        let edit_open = move |value: String| {
            last_action.set(format!("Edit menu selected: {}", value));
        };

        rsx! {
            div { class: "pb-24", // Add a big vertical margin
                Menubar {
                    // File Menu
                    MenubarMenu {
                        index: 0_usize,
                        MenubarTrigger { "File" }
                        MenubarContent {
                            MenubarItem {
                                index: 0_usize,
                                value: "new".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                "New"
                            }
                            MenubarItem {
                                index: 1_usize,
                                value: "open".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                "Open"
                            }
                            MenubarItem {
                                index: 2_usize,
                                value: "save".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                "Save"
                            }
                        }
                    }
                    // Edit Menu
                    MenubarMenu {
                        index: 1_usize,
                        MenubarTrigger { "Edit" }
                        MenubarContent {
                            MenubarItem {
                                index: 0_usize,
                                value: "cut".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                "Cut"
                            }
                            MenubarItem {
                                index: 1_usize,
                                value: "copy".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                "Copy"
                            }
                            MenubarItem {
                                index: 2_usize,
                                value: "paste".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                "Paste"
                            }
                        }
                    }
                }

                // Display last action
                if !last_action().is_empty() {
                    div {
                        class: "mt-4 p-2 rounded bg-card text-sm",
                        "Last action: {last_action}"
                    }
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod disabled {
    // ANCHOR: disabled
    use dioxus::prelude::*;
    use lumen_blocks::components::menubar::{
        Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
    };

    #[component]
    pub fn DisabledMenubarExample() -> Element {
        let mut last_action = use_signal(|| String::new());

        let handle_select = move |value: String| {
            last_action.set(format!("Menu item selected: {}", value));
        };

        rsx! {
            div { class: "pb-24", // Add a big vertical margin
                Menubar {
                    // Active Menu
                    MenubarMenu {
                        index: 0_usize,
                        MenubarTrigger { "Active Menu" }
                        MenubarContent {
                            MenubarItem {
                                index: 0_usize,
                                value: "item1".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                "Item 1"
                            }
                            MenubarItem {
                                index: 1_usize,
                                value: "item2".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                "Item 2"
                            }
                        }
                    }
                    // Disabled Menu
                    MenubarMenu {
                        index: 1_usize,
                        MenubarTrigger {
                            class: Some("opacity-50 pointer-events-none".to_string()),
                            "Disabled Menu"
                        }
                        MenubarContent {
                            MenubarItem {
                                index: 0_usize,
                                value: "disabled1".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                "Disabled Item 1"
                            }
                        }
                    }
                    // Menu with Disabled Items
                    MenubarMenu {
                        index: 2_usize,
                        MenubarTrigger { "Mixed Menu" }
                        MenubarContent {
                            MenubarItem {
                                index: 0_usize,
                                value: "active".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                "Active Item"
                            }
                            MenubarItem {
                                index: 1_usize,
                                value: "disabled".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                class: Some("opacity-50 pointer-events-none".to_string()),
                                "Disabled Item"
                            }
                        }
                    }
                }

                // Display last action
                if !last_action().is_empty() {
                    div {
                        class: "mt-4 p-2 rounded bg-card text-sm",
                        "Last action: {last_action}"
                    }
                }
            }
        }
    }
    // ANCHOR_END: disabled
}

pub mod with_icons {
    // ANCHOR: with_icons
    use dioxus::prelude::*;
    use lucide_dioxus::{Clipboard, Copy, FileText, FolderOpen, Save, Scissors};
    use lumen_blocks::components::menubar::{
        Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
    };

    #[component]
    pub fn MenubarWithIconsExample() -> Element {
        let mut last_action = use_signal(|| String::new());

        let file_open = move |value: String| {
            last_action.set(format!("File menu selected: {}", value));
        };

        let edit_open = move |value: String| {
            last_action.set(format!("Edit menu selected: {}", value));
        };

        rsx! {
            div { class: "w-full pb-24", // Add a big vertical margin
                Menubar {
                    // File Menu
                    MenubarMenu {
                        index: 0_usize,
                        MenubarTrigger { "File" }
                        MenubarContent {
                            MenubarItem {
                                index: 0_usize,
                                value: "new".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                FileText { size: 16 }
                                span { "New" }
                            }
                            MenubarItem {
                                index: 1_usize,
                                value: "open".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                FolderOpen { size: 16 }
                                span { "Open" }
                            }
                            MenubarItem {
                                index: 2_usize,
                                value: "save".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                Save { size: 16 }
                                span { "Save" }
                            }
                        }
                    }
                    // Edit Menu
                    MenubarMenu {
                        index: 1_usize,
                        MenubarTrigger { "Edit" }
                        MenubarContent {
                            MenubarItem {
                                index: 0_usize,
                                value: "cut".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                Scissors { size: 16 }
                                span { "Cut" }
                            }
                            MenubarItem {
                                index: 1_usize,
                                value: "copy".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                Copy { size: 16 }
                                span { "Copy" }
                            }
                            MenubarItem {
                                index: 2_usize,
                                value: "paste".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                Clipboard { size: 16 }
                                span { "Paste" }
                            }
                        }
                    }
                }

                // Display selected action with icon
                if !last_action().is_empty() {
                    div {
                        class: "mt-4 p-2 rounded bg-card text-sm",
                        "Selected: " strong { "{last_action()}" }
                    }
                }
            }
        }
    }
    // ANCHOR_END: with_icons
}
