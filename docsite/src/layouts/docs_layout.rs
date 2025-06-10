use dioxus::prelude::*;
use dioxus_router::prelude::{Link, Outlet, use_route};
use crate::Route;
use docs::docs::router_01::{BookRoute, LAZY_BOOK};
use mdbook_shared::SummaryItem;

// Signal to control mobile sidebar visibility
pub(crate) static SHOW_SIDEBAR: GlobalSignal<bool> = Signal::global(|| false);

#[component]
pub fn DocsLayout() -> Element {
    rsx! {
        div { class: "w-full text-sm border-b dark:border-[#a4a9ac7d] border-gray-300",
            div { class: "flex flex-row justify-center dark:text-[#dee2e6] font-light lg:gap-12",
                DocsLeftNav {}
                DocsContent {}
                DocsRightNav {}
            }
        }
    }
}

#[component]
fn DocsLeftNav() -> Element {
    let route = use_route::<Route>();
    
    // Extract the current BookRoute from the Route enum
    let current_book_route = match route {
        Route::Docs01 { child } => Some(child),
        _ => None,
    };
    
    let toggle_sidebar = move |_| *SHOW_SIDEBAR.write() = *SHOW_SIDEBAR.read();
    let is_sidebar_visible = *SHOW_SIDEBAR.read();
    
    // Get the book structure from LAZY_BOOK
    let book = &*LAZY_BOOK;
    
    // Combine all chapters for navigation
    let chapters = vec![
        &book.summary.prefix_chapters,
        &book.summary.numbered_chapters,
        &book.summary.suffix_chapters,
    ];

    rsx! {
        div { 
            class: "min-w-[240px] pt-12 pb-16 dark:border-r border-r border-[#a4a9ac7d] sticky top-16 self-start h-[calc(100vh-64px)] overflow-auto",
            class: if is_sidebar_visible { "block" } else { "hidden lg:block" },
            div { class: "pr-8",
                div { class: "flex justify-between items-center mb-4 px-6",
                    h3 { class: "font-bold", "Documentation" }
                    
                    // Mobile close button
                    button {
                        class: "lg:hidden p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700",
                        onclick: toggle_sidebar,
                        "×"
                    }
                }
                
                // Dynamic navigation based on the book structure
                nav { class: "pl-2 pb-2 text-base sm:block text-gray-600 dark:text-gray-400 pr-2 space-y-1",
                    for chapter_list in chapters.into_iter().flatten() {
                        if let Some(link) = chapter_list.maybe_link() {
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
                    class: "font-semibold hover:text-sky-500 dark:hover:text-sky-400 dark:text-gray-100 text-gray-700",
                    active_class: "text-sky-600 dark:text-sky-400",
                    div { class: "flex items-center justify-between pb-2",
                        h3 { "{link.name}" }
                        
                        if has_children {
                            button {
                                onclick: move |e| {
                                    e.stop_propagation();
                                    expanded.toggle();
                                },
                                class: "px-2",
                                if expanded() { "▼" } else { "▶" }
                            }
                        }
                    }
                }
            }
            
            if has_children && expanded() {
                ul { class: "ml-1 space-y-1 border-l border-gray-200 dark:border-gray-700 pl-4",
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
        li { class: "rounded-md hover:text-sky-500 dark:hover:text-sky-400",
            if let Some(url) = &link.location {
                Link {
                    to: Route::Docs01 { child: *url },
                    onclick: move |_| {
                        if has_children {
                            expanded.toggle();
                        }
                        *SHOW_SIDEBAR.write() = false;
                    },
                    class: "flex items-center justify-between py-1",
                    active_class: "text-sky-600 dark:text-sky-400",
                    span { "{link.name}" }
                    
                    if has_children {
                        span { class: "ml-2", if expanded() { "▼" } else { "▶" } }
                    }
                }
            }
            
            if has_children && expanded() {
                ul { class: "ml-2 mt-1 space-y-1 border-l border-gray-200 dark:border-gray-700 pl-4",
                    for child in link.nested_items.iter() {
                        SidebarChapter { chapter: child, current_route }
                    }
                }
            }
        }
    }
}

#[component]
fn DocsContent() -> Element {
    let toggle_sidebar = move |_| *SHOW_SIDEBAR.write() = *SHOW_SIDEBAR.read();

    rsx! {
        div { 
            class: "flex-1 max-w-[65ch] pt-12 pb-16 px-6",
            
            // Mobile menu button
            button {
                class: "lg:hidden mb-4 p-2 rounded-md bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700",
                onclick: toggle_sidebar,
                "Menu"
            }
            
            // This is where the current route's content will be rendered
            Outlet::<Route> {}
        }
    }
}

#[component]
fn DocsRightNav() -> Element {
    let route = use_route::<Route>();
    
    // Extract the current BookRoute from the Route enum
    let current_book_route = match route {
        Route::Docs01 { child } => Some(child),
        _ => None,
    };
    
    // Get sections from the current page
    let sections = current_book_route.map(|route| route.sections()).unwrap_or_default();

    rsx! {
        div { class: "hidden xl:block min-w-[240px] pt-12 pb-16 dark:border-l border-l border-[#a4a9ac7d] sticky top-16 self-start h-[calc(100vh-64px)] overflow-auto",
            div { class: "pl-8",
                h3 { class: "font-bold mb-4", "On This Page" }
                
                // Page sections navigation
                ul { class: "space-y-1 text-sm",
                    for section in sections.iter().skip(1) {
                        li { 
                            class: if section.level == 1 { "" as &str }
                                else if section.level == 2 { "pl-2" }
                                else if section.level == 3 { "pl-4" }
                                else { "pl-6" },
                            a { 
                                class: "block py-1 hover:text-blue-500", 
                                href: "#{section.id}",
                                "{section.title}" 
                            }
                        }
                    }
                }
                
                // Edit page link
                div { class: "mt-8 pt-4 border-t border-gray-200 dark:border-gray-700",
                    a {
                        class: "text-sm flex items-center text-gray-600 dark:text-gray-400 hover:text-blue-500 dark:hover:text-blue-400",
                        href: "https://github.com/DioxusLabs/dioxus-blocks/edit/main/docs",
                        svg {
                            class: "w-4 h-4 mr-2",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 20 20",
                            "fill": "currentColor",
                            path {
                                "d": "M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"
                            }
                        }
                        "Edit this page"
                    }
                }
            }
        }
    }
}
