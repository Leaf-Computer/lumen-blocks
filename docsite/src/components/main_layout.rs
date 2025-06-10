use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;

use crate::Route;
use crate::components::navbar::Navbar;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            // Use the existing Navbar component
            Navbar {}
            
            // Main content area
            main { class: "container mx-auto px-4 py-8",
                // This is where child routes will be rendered
                Outlet::<Route> {}
            }
            
            // Simple footer
            footer { class: "py-6 text-center text-gray-500 dark:text-gray-400",
                p { "© 2023 Your Application. All rights reserved." }
            }
        }
    }
}