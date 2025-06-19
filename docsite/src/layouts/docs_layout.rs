use dioxus::prelude::*;
use dioxus::document;
use dioxus_router::prelude::{Link, Outlet, use_route};
use crate::Route;
use docs::docs::router_01::{BookRoute, LAZY_BOOK};
use mdbook_shared::SummaryItem;

// Signal to control mobile sidebar visibility
pub(crate) static SHOW_SIDEBAR: GlobalSignal<bool> = Signal::global(|| false);

#[component]
pub fn DocsLayout() -> Element {
    let route = use_route::<Route>();
    
    // Extract the current BookRoute from the Route enum
    let current_book_route = match route {
        Route::Docs01 { child } => Some(child),
        _ => None,
    };
    
    // Generate a dynamic title based on the current page
    let title = if let Some(book_route) = current_book_route {
        // Get the title from the book's structure
        let book = &*LAZY_BOOK;
        
        // Helper function to find a title for a route
        fn find_title(items: &[SummaryItem<BookRoute>], route: &BookRoute) -> Option<String> {
            for item in items {
                if let Some(link) = item.maybe_link() {
                    // Check if this item matches the route
                    if let Some(loc) = &link.location {
                        if loc == route {
                            return Some(link.name.clone());
                        }
                    }
                    
                    // Check nested items
                    if !link.nested_items.is_empty() {
                        if let Some(title) = find_title(&link.nested_items, route) {
                            return Some(title);
                        }
                    }
                }
            }
            None
        }
        
        // Search all chapters for the title
        let chapters = vec![
            &book.summary.prefix_chapters,
            &book.summary.numbered_chapters,
            &book.summary.suffix_chapters,
        ];
        
        let page_title = chapters
            .iter()
            .flat_map(|&chapters| find_title(chapters, &book_route))
            .next()
            .unwrap_or_else(|| "Documentation".to_string());
            
        format!("Laminar Blocks - {}", page_title)
    } else {
        "Laminar Blocks - Documentation".to_string()
    };
    
    rsx! {
        document::Title { "{title}" }
        div { class: "w-full text-sm border-b border-border relative bg-background",
            div { class: "flex flex-row justify-center text-foreground font-light lg:gap-12",
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
            class: "min-w-[240px] pt-12 pb-16 border-r border-border sticky top-16 self-start h-[calc(100vh-64px)] overflow-auto bg-card/50 backdrop-blur-sm",
            class: if is_sidebar_visible { "block" } else { "hidden lg:block" },
            div { class: "pr-8",
                div { class: "flex justify-between items-center mb-4",
                    h3 { class: "text-sm text-muted-foreground text-foreground", "Documentation" }
                }
                
                // Dynamic navigation based on the book structure
                nav { class: "pl-2 pb-2 text-base sm:block text-muted-foreground pr-2 space-y-2",
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
                        h3 { class: "hover:underline",
                            "{link.name}"
                        }
                        
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
                        *SHOW_SIDEBAR.write() = false;
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

#[component]
fn DocsContent() -> Element {
    rsx! {
        div { 
            class: "
                flex-1 max-w-[80ch] w-full pt-12 pb-16 px-6
                text-foreground bg-background

                /* Headings */
                [&_h1]:mt-12 [&_h1]:mb-3 [&_h1]:text-3xl [&_h1]:font-bold [&_h1]:text-foreground
                [&_h2]:mt-12 [&_h2]:mb-3 [&_h2]:text-2xl [&_h2]:font-semibold [&_h2]:text-foreground
                [&_h3]:mt-12 [&_h3]:mb-3 [&_h3]:text-xl [&_h3]:font-medium [&_h3]:text-foreground
                [&_h4]:mt-12 [&_h4]:mb-3 [&_h4]:text-lg [&_h4]:font-medium [&_h4]:text-foreground
                [&_h5]:mt-10 [&_h5]:mb-2 [&_h5]:text-base [&_h5]:font-medium [&_h5]:text-foreground
                [&_h6]:mt-8 [&_h6]:mb-2 [&_h6]:text-sm [&_h6]:font-medium [&_h6]:text-foreground

                /* Paragraphs */
                [&_p]:my-3

                /* Lists */
                [&_ul]:list-disc [&_ul]:ml-6
                [&_li]:my-2

                /* Links */
                [&_a]:text-primary [&_a]:transition-colors [&_a]:underline [&_a:hover]:text-foreground [&_a]:font-bold [&_a]:decoration-transparent [&_a:hover]:decoration-foreground

                /* Text formatting */
                [&_strong]:font-bold
            ",
            
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
        div { class: "hidden xl:block min-w-[240px] pt-12 pb-16 border-l border-border sticky top-16 self-start h-[calc(100vh-64px)] overflow-auto bg-card/50 backdrop-blur-sm",
            div { class: "pl-8",
                h3 { class: "font-bold mb-4 text-foreground", "On This Page" }
                
                // Page sections navigation
                ul { class: "space-y-2 text-sm",
                    for section in sections.iter().skip(1) {
                        li { 
                            class: if section.level == 1 { "" as &str }
                                else if section.level == 2 { "pl-2" }
                                else if section.level == 3 { "pl-4" }
                                else { "pl-6" },
                            a { 
                                class: "block py-1 text-muted-foreground hover:text-primary transition-colors rounded px-2 hover:bg-muted/50", 
                                href: "#{section.id}",
                                "{section.title}" 
                            }
                        }
                    }
                }
                
                // Edit page link
                div { class: "mt-8 pt-4 border-t border-border",
                    a {
                        class: "text-sm flex items-center text-muted-foreground hover:text-primary transition-colors rounded p-1 hover:bg-muted/50",
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
