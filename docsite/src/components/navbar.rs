use dioxus::prelude::*;
use ::docs::docs::router_01::{BookRoute, LAZY_BOOK};
use laminar_blocks::components::side_sheet::*;
use laminar_blocks::components::button::{Button, ButtonVariant};
use lucide_dioxus::Menu;
use mdbook_shared::SummaryItem;
use crate::Route;
use crate::LAMINAR_LOGO_SMALL;
use docs::docs;

#[component]
pub fn Navbar() -> Element {
    let route = use_route::<Route>();
    
    // Extract the current BookRoute from the Route enum
    let current_book_route = match route {
        Route::Docs01 { child } => Some(child),
        _ => None,
    };
    
    // Get the book structure from LAZY_BOOK
    let book = &*LAZY_BOOK;
    
    // Combine all chapters for navigation
    let chapters = vec![
        &book.summary.prefix_chapters,
        &book.summary.numbered_chapters,
        &book.summary.suffix_chapters,
    ];
    
    rsx! {
        nav {
            class: "bg-card/80 backdrop-blur-sm border-b border-border px-6 py-4 sticky top-0 z-50",
            div {
                class: "max-w-6xl mx-auto flex items-center justify-between",
                // Logo section
                Link {
                    to: Route::Home {},
                    class: "text-foreground hover:text-primary transition-colors",
                    div {
                        class: "flex items-center gap-2",
                        img { class: "w-8 h-8", src: LAMINAR_LOGO_SMALL }
                        span { class: "text-xl font-bold text-foreground", "Laminar Blocks" }
                    }
                }
                // Desktop navigation links
                div {
                    class: "hidden md:flex items-center gap-6",
                    Link {
                        to: Route::Home {},
                        class: "text-foreground hover:text-primary transition-colors",
                        "Home"
                    }
                    Link {
                        to: Route::Docs01 { child: docs::router_01::BookRoute::Index { section: Default::default() } },
                        class: "text-foreground hover:text-primary transition-colors",
                        "Docs"
                    }
                }
                div { class: "md:hidden",
                    SideSheetTrigger {
                        Button {
                            variant: ButtonVariant::Ghost,
                            Menu { class: "h-6 w-6" }
                        }
                    }
                }
            }
        }
        nav {
            class: "md:hidden",
            SideSheetContent {
                class: "p-6 w-64 h-full flex flex-col space-y-8",
                SideSheetCloseButton {},
                nav {
                    class: "flex flex-col space-y-3",
                    div { class: "text-muted-foreground text-xs", 
                        "Menu"
                    }
                    Link {
                        to: Route::Home {},
                        class: "text-foreground hover:text-primary transition-colors",
                        "Home"
                    }
                    Link {
                        to: Route::Docs01 { child: docs::router_01::BookRoute::Index { section: Default::default() } },
                        class: "text-foreground hover:text-primary transition-colors",
                        "Docs"
                    }
                }
                nav {
                    class: "flex flex-col space-y-3",
                    div { class: "text-muted-foreground text-xs", 
                        "Docs"
                    }
                    for chapter_list in chapters.into_iter().flatten() {
                        if let Some(_link) = chapter_list.maybe_link() {
                            SidebarSection { 
                                chapter: chapter_list,
                                current_route: current_book_route
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarSection(chapter: &'static SummaryItem<BookRoute>, current_route: Option<BookRoute>) -> Element {
    let link = chapter.maybe_link().context("Could not get link")?;
    
    // Check if this section or any of its children is active
    let is_active = current_route.map(|route| 
        link.location.as_ref().map(|loc| *loc == route).unwrap_or(false)
    ).unwrap_or(false);
    
    let has_children = !link.nested_items.is_empty();
    let mut expanded = use_signal(|| is_active);

    rsx! {
        div { class: "full-chapter",
            if let Some(url) = &link.location {
                Link {
                    to: Route::Docs01 { child: *url },
                    class: "font-semibold text-foreground hover:text-primary transition-colors",
                    active_class: "text-primary",
                    div { class: "flex items-center justify-between pb-2",
                        h3 { "{link.name}" }
                        
                        if has_children {
                            button {
                                onclick: move |e| {
                                    e.stop_propagation();
                                    expanded.toggle();
                                },
                                class: "px-2 text-muted-foreground",
                                if expanded() { "▼" } else { "▶" }
                            }
                        }
                    }
                }
            }
            
            if has_children && expanded() {
                ul { class: "ml-1 space-y-1 border-l border-border pl-4",
                    for chapter in link.nested_items.iter() {
                        SidebarChapter { 
                            chapter,
                            current_route
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarChapter(chapter: &'static SummaryItem<BookRoute>, current_route: Option<BookRoute>) -> Element {
    let link = chapter.maybe_link().context("Could not get link")?;
    
    // Check if this item is active
    let is_active = current_route.map(|route| 
        link.location.as_ref().map(|loc| *loc == route).unwrap_or(false)
    ).unwrap_or(false);
    
    let has_children = !link.nested_items.is_empty();
    let mut expanded = use_signal(|| is_active);

    rsx! {
        li { class: "rounded-md",
            if let Some(url) = &link.location {
                Link {
                    to: Route::Docs01 { child: *url },
                    onclick: move |_| {
                        if has_children {
                            expanded.toggle();
                        }
                    },
                    class: "flex items-center justify-between py-1 text-foreground hover:text-primary transition-colors",
                    active_class: "text-primary",
                    span { "{link.name}" }
                    
                    if has_children {
                        span { class: "ml-2 text-muted-foreground", if expanded() { "▼" } else { "▶" } }
                    }
                }
            }
            
            if has_children && expanded() {
                ul { class: "ml-2 mt-1 space-y-1 border-l border-border pl-4",
                    for child in link.nested_items.iter() {
                        SidebarChapter { chapter: child, current_route }
                    }
                }
            }
        }
    }
}
