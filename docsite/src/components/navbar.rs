use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;
use crate::LAMINAR_LOGO;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "bg-card border-b border-border px-6 py-4",
            div { class: "max-w-6xl mx-auto flex items-center justify-between",
                div { class: "flex items-center gap-2",
                    img { class: "w-8 h-8", src: LAMINAR_LOGO }
                    span { class: "text-xl font-bold text-foreground", "Laminar Blocks" }
                }
                
                div { class: "flex items-center gap-6",
                    Link { 
                        to: Route::Home {},
                        class: "text-foreground hover:text-primary transition-colors",
                        "Home" 
                    }
                    Link { 
                        to: Route::Components {},
                        class: "text-foreground hover:text-primary transition-colors",
                        "Components" 
                    }
                    Link { 
                        to: Route::Dashboard {},
                        class: "text-foreground hover:text-primary transition-colors",
                        "Dashboard" 
                    }
                }
            }
        }
    }
}
