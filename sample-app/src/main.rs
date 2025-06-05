use dioxus::prelude::*;
use laminar_blocks::components::menubar::{
    Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem,
};
use crate::document::eval;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
            MenubarExample {}
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
pub fn MenubarExample() -> Element {
    let file_open = move |value: String| {
        eval(&format!("console.log('Selected: {}')", value));
    };

    let edit_open = move |value: String| {
        eval(&format!("console.log('Selected: {}')", value));
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
