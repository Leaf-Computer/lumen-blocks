use dioxus::prelude::*;
use dioxus_router::{use_route, Outlet};
use lumen_blocks::components::side_sheet::SideSheet;

use crate::components::navbar::Navbar;
use crate::Route;

#[component]
pub fn MainLayout() -> Element {
    let route = use_route::<Route>();
    let title = match route {
        Route::Home { .. } => "Lumen Blocks - Home",
        Route::Docs { .. } => "Lumen Blocks - Documentation",
        Route::Err404 { .. } => "Lumen Blocks - Page Not Found",
    };

    rsx! {
        document::Title { "{title}" }
        SideSheet {
            div { class: "min-h-screen relative",
                // Use the existing Navbar component
                Navbar {}

                // Main content area
                main { class: "container mx-auto",
                    // This is where child routes will be rendered
                    Outlet::<Route> {}
                }

                // Simple footer
                footer { class: "py-6 text-center text-gray-500 dark:text-gray-400",
                    p {
                        "Crafted with "
                        span { class: "text-red-500", "♥" }
                        " by "
                        a {
                            class: "hover:underline",
                            target: "_blank",
                            href: "https://leaf.computer",
                            "Leaf Computer"
                        }
                    }
                }
            }
        }
    }
}
