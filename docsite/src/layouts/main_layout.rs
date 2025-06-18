use dioxus::prelude::*;
use dioxus_router::prelude::{Outlet, use_route};
use laminar_blocks::components::side_sheet::SideSheet;

use crate::Route;
use crate::components::navbar::Navbar;

#[component]
pub fn MainLayout() -> Element {
    let route = use_route::<Route>();
    let title = match route {
        Route::Home { .. } => "Laminar Blocks - Home",
        Route::Docs01 { .. } => "Laminar Blocks - Documentation",
        Route::Err404 { .. } => "Laminar Blocks - Page Not Found",
        _ => "Laminar Blocks",
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
                    p { "© 2025 Leaf Computer Corporation. All rights reserved." }
                }
            }
        }
    }
}
