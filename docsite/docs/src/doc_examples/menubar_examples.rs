#![allow(non_snake_case)]

pub use basic::BasicMenubarExample;
pub use disabled::DisabledMenubarExample;
pub use custom_styling::CustomStyledMenubarExample;
pub use with_icons::MenubarWithIconsExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::menubar::{
        Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem,
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
                        index: 0,
                        MenubarTrigger { "File" }
                        MenubarContent {
                            MenubarItem {
                                value: "new".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                "New"
                            }
                            MenubarItem {
                                value: "open".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                "Open"
                            }
                            MenubarItem {
                                value: "save".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                "Save"
                            }
                        }
                    }
                    // Edit Menu
                    MenubarMenu {
                        index: 1,
                        MenubarTrigger { "Edit" }
                        MenubarContent {
                            MenubarItem {
                                value: "cut".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                "Cut"
                            }
                            MenubarItem {
                                value: "copy".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                "Copy"
                            }
                            MenubarItem {
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
    use laminar_blocks::components::menubar::{
        Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem,
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
                        index: 0,
                        MenubarTrigger { "Active Menu" }
                        MenubarContent {
                            MenubarItem {
                                value: "item1".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                "Item 1"
                            }
                            MenubarItem {
                                value: "item2".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                "Item 2"
                            }
                        }
                    }
                    // Disabled Menu
                    MenubarMenu {
                        index: 1,
                        MenubarTrigger { 
                            class: Some("opacity-50 pointer-events-none".to_string()), 
                            "Disabled Menu" 
                        }
                        MenubarContent {
                            MenubarItem {
                                value: "disabled1".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                "Disabled Item 1"
                            }
                        }
                    }
                    // Menu with Disabled Items
                    MenubarMenu {
                        index: 2,
                        MenubarTrigger { "Mixed Menu" }
                        MenubarContent {
                            MenubarItem {
                                value: "active".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                "Active Item"
                            }
                            MenubarItem {
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

pub mod custom_styling {
    // ANCHOR: custom_styling
    use dioxus::prelude::*;
    use laminar_blocks::components::menubar::{
        Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem,
    };
    
    #[component]
    pub fn CustomStyledMenubarExample() -> Element {
        let mut last_action = use_signal(|| String::new());
        
        let handle_select = move |value: String| {
            last_action.set(format!("Menu item selected: {}", value));
        };

        rsx! {
            div { class: "pb-24", // Add a big vertical margin
                Menubar {
                    class: Some("bg-secondary rounded-md p-1 gap-1".to_string()),
                    // File Menu
                    MenubarMenu {
                        index: 0,
                        MenubarTrigger { 
                            class: Some("px-3 py-1.5 text-sm font-medium rounded-sm hover:bg-primary hover:text-primary-foreground".to_string()),
                            "File" 
                        }
                        MenubarContent {
                            class: Some("rounded-md border bg-popover p-2 text-popover-foreground shadow-md".to_string()),
                            MenubarItem {
                                value: "new".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                class: Some("px-2 py-1.5 text-sm rounded-sm hover:bg-accent hover:text-accent-foreground".to_string()),
                                "New"
                            }
                            MenubarItem {
                                value: "open".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                class: Some("px-2 py-1.5 text-sm rounded-sm hover:bg-accent hover:text-accent-foreground".to_string()),
                                "Open"
                            }
                        }
                    }
                    // Edit Menu
                    MenubarMenu {
                        index: 1,
                        MenubarTrigger { 
                            class: Some("px-3 py-1.5 text-sm font-medium rounded-sm hover:bg-primary hover:text-primary-foreground".to_string()),
                            "Edit" 
                        }
                        MenubarContent {
                            class: Some("rounded-md border bg-popover p-2 text-popover-foreground shadow-md".to_string()),
                            MenubarItem {
                                value: "cut".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                class: Some("px-2 py-1.5 text-sm rounded-sm hover:bg-accent hover:text-accent-foreground".to_string()),
                                "Cut"
                            }
                            MenubarItem {
                                value: "copy".to_string(),
                                on_select: Callback::new(handle_select.clone()),
                                class: Some("px-2 py-1.5 text-sm rounded-sm hover:bg-accent hover:text-accent-foreground".to_string()),
                                "Copy"
                            }
                        }
                    }
                }
                
                // Display selected action with custom styling
                if !last_action().is_empty() {
                    div { 
                        class: "p-3 bg-secondary/20 rounded-md mt-4",
                        "Selected: " strong { "{last_action()}" }
                    }
                }
            }
        }
    }
    // ANCHOR_END: custom_styling
}

pub mod with_icons {
    // ANCHOR: with_icons
    use dioxus::prelude::*;
    use laminar_blocks::components::menubar::{
        Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem,
    };
    use lucide_dioxus::{FileText, FolderOpen, Save, Scissors, Copy, Clipboard};
    
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
            div { class: "pb-24", // Add a big vertical margin
                Menubar {
                    // File Menu
                    MenubarMenu {
                        index: 0,
                        MenubarTrigger { "File" }
                        MenubarContent {
                            MenubarItem {
                                value: "new".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                FileText { size: 16 }
                                span { "New" }
                            }
                            MenubarItem {
                                value: "open".to_string(),
                                on_select: Callback::new(file_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                FolderOpen { size: 16 }
                                span { "Open" }
                            }
                            MenubarItem {
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
                        index: 1,
                        MenubarTrigger { "Edit" }
                        MenubarContent {
                            MenubarItem {
                                value: "cut".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                Scissors { size: 16 }
                                span { "Cut" }
                            }
                            MenubarItem {
                                value: "copy".to_string(),
                                on_select: Callback::new(edit_open.clone()),
                                class: Some("flex items-center gap-2".to_string()),
                                Copy { size: 16 }
                                span { "Copy" }
                            }
                            MenubarItem {
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
