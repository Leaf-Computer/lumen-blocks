use dioxus::prelude::*;
use dioxus_blocks::components::menubar::{
    Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem,
};
use dioxus::document::eval;

fn main() {
    #[cfg(debug_assertions)]
    {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        log::info!("Logger initialized in debug mode");
    }

    dioxus::launch(MenubarExample);
}

#[component]
pub fn MenubarExample() -> Element {
    let file_open = move |value: String| {
        eval(&format!("console.log('Selected: {}')", value));
        log::info!("File menu selected: {}", value);
    };

    let edit_open = move |value: String| {
        eval(&format!("console.log('Selected: {}')", value));
        log::info!("Edit menu selected: {}", value);
    };

    rsx! {
        div { class: "menubar-example p-5 flex flex-col gap-8 items-start bg-background min-h-[300px]",
            h2 { class: "mb-4 text-xl font-semibold", "Menubar Example" }
            Menubar {
                class: None,
                // File Menu
                MenubarMenu {
                    index: 0,
                    MenubarTrigger { class: None, "File" }
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
                    MenubarTrigger { class: None, "Edit" }
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
                // View Menu (disabled example)
                MenubarMenu {
                    index: 2,
                    MenubarTrigger { class: Some("opacity-50 pointer-events-none".to_string()), "View (Disabled)" }
                }
            }
            p { class: "text-muted-foreground text-sm", "Try clicking the menu items to see log output." }
        }
    }
}
