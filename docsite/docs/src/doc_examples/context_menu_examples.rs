#![allow(non_snake_case)]

use lucide_dioxus::{Pencil, Plus, Trash, Settings, Share2, Search, X};

pub use basic::BasicContextMenuExample;
pub use with_labels::ContextMenuWithLabelsExample;
pub use with_checkboxes::ContextMenuWithCheckboxesExample;
pub use with_radio::ContextMenuWithRadioExample;
pub use disabled::DisabledContextMenuExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::context_menu::*;
    use lucide_dioxus::{Pencil, Plus, Trash};
    
    #[component]
    pub fn BasicContextMenuExample() -> Element {
        let mut selected_action = use_signal(|| String::new());
        
        rsx! {
            div { class: "space-y-4",
                ContextMenu {
                    ContextMenuTrigger {
                        div { 
                            class: "flex h-32 w-full items-center justify-center rounded-md border border-dashed border-border bg-muted/50 text-sm",
                            "Right-click here for basic menu"
                        }
                    }
                    
                    ContextMenuContent { width: "w-48",
                        ContextMenuItem {
                            value: ReadOnlySignal::new(Signal::new("edit".to_string())),
                            index: ReadOnlySignal::new(Signal::new(0)),
                            icon: rsx! { Pencil { class: "h-4 w-4" } },
                            on_select: move |value| selected_action.set(value),
                            "Edit"
                        }
                        
                        ContextMenuItem {
                            value: ReadOnlySignal::new(Signal::new("copy".to_string())),
                            index: ReadOnlySignal::new(Signal::new(1)),
                            icon: rsx! { Plus { class: "h-4 w-4" } },
                            on_select: move |value| selected_action.set(value),
                            "Copy"
                        }
                        
                        ContextMenuSeparator {}
                        
                        ContextMenuItem {
                            value: ReadOnlySignal::new(Signal::new("delete".to_string())),
                            index: ReadOnlySignal::new(Signal::new(2)),
                            destructive: ReadOnlySignal::new(Signal::new(true)),
                            icon: rsx! { Trash { class: "h-4 w-4" } },
                            on_select: move |value| selected_action.set(value),
                            "Delete"
                        }
                    }
                }
                
                if !selected_action().is_empty() {
                    div { class: "mt-4 p-3 bg-muted rounded-md",
                        "Last action: " strong { "{selected_action()}" }
                    }
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod with_labels {
    // ANCHOR: with_labels
    use dioxus::prelude::*;
    use laminar_blocks::components::context_menu::*;
    use lucide_dioxus::{Search, Share2, Settings};
    
    #[component]
    pub fn ContextMenuWithLabelsExample() -> Element {
        let mut selected_action = use_signal(|| String::new());
        
        rsx! {
            div { class: "space-y-4",
                ContextMenu {
                    ContextMenuTrigger {
                        div { 
                            class: "flex h-32 w-full items-center justify-center rounded-md border border-dashed border-border bg-muted/50 text-sm",
                            "Right-click for labeled menu"
                        }
                    }
                    
                    ContextMenuContent { width: "w-56",
                        ContextMenuLabel { "Actions" }
                        ContextMenuItem {
                            value: ReadOnlySignal::new(Signal::new("download".to_string())),
                            index: ReadOnlySignal::new(Signal::new(0)),
                            icon: rsx! { Search { class: "h-4 w-4" } },
                            on_select: move |value| selected_action.set(value),
                            "Download"
                        }
                        
                        ContextMenuItem {
                            value: ReadOnlySignal::new(Signal::new("share".to_string())),
                            index: ReadOnlySignal::new(Signal::new(1)),
                            icon: rsx! { Share2 { class: "h-4 w-4" } },
                            on_select: move |value| selected_action.set(value),
                            "Share"
                        }
                        
                        ContextMenuSeparator {}
                        
                        ContextMenuLabel { "Settings" }
                        ContextMenuItem {
                            value: ReadOnlySignal::new(Signal::new("preferences".to_string())),
                            index: ReadOnlySignal::new(Signal::new(2)),
                            icon: rsx! { Settings { class: "h-4 w-4" } },
                            on_select: move |value| selected_action.set(value),
                            "Preferences"
                        }
                    }
                }
                
                if !selected_action().is_empty() {
                    div { class: "mt-4 p-3 bg-muted rounded-md",
                        "Last action: " strong { "{selected_action()}" }
                    }
                }
            }
        }
    }
    // ANCHOR_END: with_labels
}

pub mod with_checkboxes {
    // ANCHOR: with_checkboxes
    use dioxus::prelude::*;
    use laminar_blocks::components::context_menu::*;
    
    #[component]
    pub fn ContextMenuWithCheckboxesExample() -> Element {
        let mut checkbox_states = use_signal(|| vec![false, false, false]);
        
        rsx! {
            div { class: "space-y-4",
                ContextMenu {
                    ContextMenuTrigger {
                        div { 
                            class: "flex h-32 w-full items-center justify-center rounded-md border border-dashed border-border bg-muted/50 text-sm",
                            "Right-click for checkbox menu"
                        }
                    }
                    
                    ContextMenuContent { width: "w-52",
                        ContextMenuLabel { "View Options" }
                        
                        ContextMenuCheckboxItem {
                            value: ReadOnlySignal::new(Signal::new("show_toolbar".to_string())),
                            index: ReadOnlySignal::new(Signal::new(0)),
                            checked: ReadOnlySignal::new(Signal::new(checkbox_states()[0])),
                            on_change: move |checked| {
                                let mut states = checkbox_states();
                                states[0] = checked;
                                checkbox_states.set(states);
                            },
                            "Show Toolbar"
                        }
                        
                        ContextMenuCheckboxItem {
                            value: ReadOnlySignal::new(Signal::new("show_sidebar".to_string())),
                            index: ReadOnlySignal::new(Signal::new(1)),
                            checked: ReadOnlySignal::new(Signal::new(checkbox_states()[1])),
                            on_change: move |checked| {
                                let mut states = checkbox_states();
                                states[1] = checked;
                                checkbox_states.set(states);
                            },
                            "Show Sidebar"
                        }
                        
                        ContextMenuCheckboxItem {
                            value: ReadOnlySignal::new(Signal::new("show_footer".to_string())),
                            index: ReadOnlySignal::new(Signal::new(2)),
                            checked: ReadOnlySignal::new(Signal::new(checkbox_states()[2])),
                            on_change: move |checked| {
                                let mut states = checkbox_states();
                                states[2] = checked;
                                checkbox_states.set(states);
                            },
                            "Show Footer"
                        }
                    }
                }
                
                div { class: "mt-4 p-3 bg-muted rounded-md space-y-1",
                    div { "Toolbar: " span { class: if checkbox_states()[0] { "text-green-600" } else { "text-red-600" }, if checkbox_states()[0] { "Enabled" } else { "Disabled" } } }
                    div { "Sidebar: " span { class: if checkbox_states()[1] { "text-green-600" } else { "text-red-600" }, if checkbox_states()[1] { "Enabled" } else { "Disabled" } } }
                    div { "Footer: " span { class: if checkbox_states()[2] { "text-green-600" } else { "text-red-600" }, if checkbox_states()[2] { "Enabled" } else { "Disabled" } } }
                }
            }
        }
    }
    // ANCHOR_END: with_checkboxes
}

pub mod with_radio {
    // ANCHOR: with_radio
    use dioxus::prelude::*;
    use laminar_blocks::components::context_menu::*;
    
    #[component]
    pub fn ContextMenuWithRadioExample() -> Element {
        let mut radio_value = use_signal(|| String::from("small"));
        let mut visibility_value = use_signal(|| String::from("visible"));
        
        rsx! {
            div { class: "space-y-4",
                ContextMenu {
                    ContextMenuTrigger {
                        div { 
                            class: "flex h-32 w-full items-center justify-center rounded-md border border-dashed border-border bg-muted/50 text-sm",
                            "Right-click for radio menu"
                        }
                    }
                    
                    ContextMenuContent { width: "w-48",
                        ContextMenuLabel { "Size" }
                        ContextMenuRadioGroup {
                            value: ReadOnlySignal::new(radio_value.clone()),
                            on_value_change: move |value| radio_value.set(value),
                            
                            ContextMenuRadioItem {
                                value: ReadOnlySignal::new(Signal::new("small".to_string())),
                                index: ReadOnlySignal::new(Signal::new(0)),
                                "Small"
                            }
                            
                            ContextMenuRadioItem {
                                value: ReadOnlySignal::new(Signal::new("medium".to_string())),
                                index: ReadOnlySignal::new(Signal::new(1)),
                                "Medium"
                            }
                            
                            ContextMenuRadioItem {
                                value: ReadOnlySignal::new(Signal::new("large".to_string())),
                                index: ReadOnlySignal::new(Signal::new(2)),
                                "Large"
                            }
                        }
                        
                        ContextMenuSeparator {}
                        
                        ContextMenuLabel { "Visibility" }
                        ContextMenuRadioGroup {
                            value: ReadOnlySignal::new(visibility_value.clone()),
                            on_value_change: move |value| visibility_value.set(value),
                            
                            ContextMenuRadioItem {
                                value: ReadOnlySignal::new(Signal::new("visible".to_string())),
                                index: ReadOnlySignal::new(Signal::new(0)),
                                "Visible"
                            }
                            
                            ContextMenuRadioItem {
                                value: ReadOnlySignal::new(Signal::new("hidden".to_string())),
                                index: ReadOnlySignal::new(Signal::new(1)),
                                "Hidden"
                            }
                        }
                    }
                }
                
                div { class: "mt-4 p-3 bg-muted rounded-md space-y-1",
                    div { "Size: " strong { "{radio_value()}" } }
                    div { "Visibility: " strong { "{visibility_value()}" } }
                }
            }
        }
    }
    // ANCHOR_END: with_radio
}

pub mod disabled {
    // ANCHOR: disabled
    use dioxus::prelude::*;
    use laminar_blocks::components::context_menu::*;
    
    #[component]
    pub fn DisabledContextMenuExample() -> Element {
        rsx! {
            div { class: "space-y-4",
                ContextMenu {
                    disabled: ReadOnlySignal::new(Signal::new(true)),
                    
                    ContextMenuTrigger {
                        div { 
                            class: "flex h-32 w-full items-center justify-center rounded-md border border-dashed border-border bg-muted/50 text-sm opacity-50",
                            "Right-click (disabled)"
                        }
                    }
                    
                    ContextMenuContent { width: "w-48",
                        ContextMenuItem {
                            value: ReadOnlySignal::new(Signal::new("disabled_action".to_string())),
                            index: ReadOnlySignal::new(Signal::new(0)),
                            "This won't work"
                        }
                    }
                }
                
                div { class: "mt-4 p-3 bg-muted rounded-md",
                    "This context menu is disabled and won't respond to right-clicks."
                }
            }
        }
    }
    // ANCHOR_END: disabled
}

// Original example for backward compatibility
pub mod example {
    use dioxus::prelude::*;
    use laminar_blocks::components::context_menu::*;
    use lucide_dioxus::{Pencil, Plus, Trash};
    
    #[component]
    pub fn ContextMenuExample() -> Element {
        let mut selected_action = use_signal(|| String::new());
        
        rsx! {
            ContextMenu {
                ContextMenuTrigger {
                    div { 
                        class: "flex h-32 w-full items-center justify-center rounded-md border border-dashed border-border bg-muted/50 text-sm",
                        "Right-click here"
                    }
                }
                
                ContextMenuContent { width: "w-48",
                    ContextMenuItem {
                        value: ReadOnlySignal::new(Signal::new("edit".to_string())),
                        index: ReadOnlySignal::new(Signal::new(0)),
                        icon: rsx! { Pencil { class: "h-4 w-4" } },
                        on_select: move |value| selected_action.set(value),
                        "Edit"
                    }
                    
                    ContextMenuItem {
                        value: ReadOnlySignal::new(Signal::new("copy".to_string())),
                        index: ReadOnlySignal::new(Signal::new(1)),
                        icon: rsx! { Plus { class: "h-4 w-4" } },
                        on_select: move |value| selected_action.set(value),
                        "Copy"
                    }
                    
                    ContextMenuSeparator {}
                    
                    ContextMenuItem {
                        value: ReadOnlySignal::new(Signal::new("delete".to_string())),
                        index: ReadOnlySignal::new(Signal::new(2)),
                        destructive: ReadOnlySignal::new(Signal::new(true)),
                        icon: rsx! { Trash { class: "h-4 w-4" } },
                        on_select: move |value| selected_action.set(value),
                        "Delete"
                    }
                }
            }
        }
    }
}