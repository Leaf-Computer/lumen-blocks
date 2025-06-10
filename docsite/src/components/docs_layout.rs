#[component]
fn DocsLayout() -> Element {
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
    
    rsx! {
        div {
            class: if SHOW_SIDEBAR() { "w-full md:w-auto" } else { "hidden" },
            class: "h-full md:block top-28 sticky md:h-[calc(100vh_-_calc(var(--spacing)_*_28))]",
            div { class: "md:flex md:flex-col md:h-full mb-2 md:text-[14px] leading-5 text-gray-700 dark:text-gray-400 space-y-2 px-4 md:px-2 py-2 md:py-0",
                DocsVersionSwitch {}
                nav { class: "
                styled-scrollbar
                pl-2 pb-2 z-20 text-base sm:block top-28
                md:w-60 lg:text-[14px] content-start text-gray-600 dark:text-gray-400 overflow-y-scroll pr-2 space-y-1",
                    DocsSidebarItem { title: "Getting Started", path: "getting-started" }
                    DocsSidebarItem { title: "Components", path: "components" }
                    DocsSidebarItem { title: "Advanced", path: "advanced" }
                    DocsSidebarItem { title: "Examples", path: "examples" }
                }
            }
        }
    }
}

#[component]
fn DocsSidebarItem(title: &'static str, path: &'static str) -> Element {
    let current_path = use_route::<Route>().to_string();
    let is_active = current_path.contains(path);
    
    rsx! {
        li { class: "rounded-md hover:text-sky-500 dark:hover:text-sky-400 flex flex-row",
            Link {
                onclick: move |_| {
                    *SHOW_SIDEBAR.write() = false;
                },
                to: format!("/docs/{path}"),
                class: "flex-grow flex flex-row items-center",
                active_class: "text-sky-600 dark:text-sky-400",
                "{title}"
            }
        }
    }
}

#[component]
fn DocsVersionSwitch() -> Element {
    let mut show_versions = use_signal(|| false);
    
    rsx! {
        div {
            tabindex: "0",
            cursor: "pointer",
            role: "button",
            onmouseleave: move |_| show_versions.set(false),
            onclick: move |_| show_versions.toggle(),
            div { class: "hover:bg-gray-100 dark:hover:bg-ghdarkmetal rounded w-full py-1",
                div { class: "grid grid-cols-[auto_1fr_auto] items-center gap-2 px-1",
                    div { class: "w-8 h-8 rounded-md border flex items-center justify-center bg-gray-50 border-gray-200 text-gray-900 dark:bg-inherit dark:text-gray-500 dark:border-gray-700 ",
                        // Placeholder for version icon
                        span { "v" }
                    }
                    div { class: "leading-snug text-xs text-left",
                        p { class: "font-medium text-gray-700 dark:text-gray-100",
                            "Using Stable Version"
                        }
                        p { class: "font-light", "0.6.0" }
                    }
                    // Placeholder for dropdown chevrons
                    span { "▼" }
                }
            }
            div {
                class: "relative w-full z-50",
                class: if !show_versions() { "hidden" },
                div { class: "absolute flex flex-col bg-white dark:bg-ghdarkmetal text-left rounded-lg border dark:border-gray-700 w-full overflow-hidden text-gray-500 dark:text-gray-100 text-xs shadow-lg",
                    DocsVersionItem { version: "0.6.0", full_version: "Version 0.6.0 - Stable" }
                    DocsVersionItem { version: "0.5.0", full_version: "Version 0.5.0 - Legacy" }
                    DocsVersionItem { version: "0.4.0", full_version: "Version 0.4.0 - Legacy" }
                }
            }
        }
    }
}

#[component]
fn DocsVersionItem(version: &'static str, full_version: &'static str) -> Element {
    rsx! {
        Link { to: format!("/docs/{version}"), active_class: "bg-gray-100 dark:bg-gray-900",
            div { class: "flex flex-row items-center hover:bg-gray-100 dark:hover:bg-gray-900 py-2 gap-2 px-2",
                span { class: "row-span-3", " " }
                div { class: "flex flex-col",
                    span { class: "text-gray-700 dark:text-gray-100 font-semibold",
                        "Version {version}"
                    }
                    span { class: "row-span-2 col-span-2", "{full_version}" }
                }
                div { class: "flex-1" }
                span {}
            }
        }
    }
}

#[component]
fn DocsContent() -> Element {
    let route = use_route::<Route>();

    rsx! {
        section {
            class: "text-gray-600 dark:text-gray-300 body-font overflow-hidden container pb-12 max-w-screen-sm px-4 pt-4 md:pt-[3.125rem] grow min-h-[100vh] ",
            class: if SHOW_SIDEBAR() { "hidden md:block" },
            div { class: "",
                DocsBreadcrumbs {}
                div { class: "flex w-full flex-wrap list-none",
                    article { class: "markdown-body", 
                        Outlet::<Route> {}
                    }
                }
                DocsNavigation {}
            }
        }
    }
}

#[component]
fn DocsBreadcrumbs() -> Element {
    let route = use_route::<Route>();
    let child = match route {
        Route::Docs01 { child } => child.clone(),
        Route::Home {} => docs::router_01::BookRoute::Index { section: Default::default() },
        Route::Components {} => docs::router_01::BookRoute::Index { section: Default::default() },
        Route::Dashboard {} => docs::router_01::BookRoute::Index { section: Default::default() },
    };

    rsx! {
        div {
            class: "flex flex-row items-center space-x-2 font-extralight pb-9",
            Link { to: "/", "Dioxus 0.6.0" }
            // Chevron placeholder
            span { ">" }
            Link {
                to: "/docs",
                "Documentation"
            }
            // Chevron placeholder
            span { ">" }
            Link {
                to: format!("/docs/{}", child),
                class: "font-semibold",
                "{child}"
            }
        }
    }
}

#[component]
fn DocsNavigation() -> Element {
    rsx! {
        div { class: "flex flex-row w-full pt-8",
            div {
                class: "text-gray-700 dark:text-gray-100 p-4 rounded text-left flex-1 ",
                div { class: "flex flex-row items-center gap-x-2 hover:text-sky-500 dark:hover:text-sky-400",
                    // Chevron left placeholder
                    span { "←" }
                    div { class: "flex flex-col",
                        span { class: "text-xs", "PREVIOUS" }
                        span { class: "font-semibold", "Getting Started" }
                    }
                }
            }

            div {
                class: "text-gray-700 dark:text-gray-100 p-4 rounded text-right flex-1",
                div { class: "flex flex-row items-center gap-x-2 justify-end hover:text-sky-500 dark:hover:text-sky-400",
                    div { class: "flex flex-col",
                        span { class: "text-xs", "NEXT" }
                        span { class: "font-semibold", "Advanced Usage" }
                    }
                    // Chevron right placeholder
                    span { "→" }
                }
            }
        }
    }
}

#[component]
fn DocsRightNav() -> Element {
    rsx! {
        div { class: "overflow-y-auto hidden xl:block top-28 px-2 h-full md:text-[14px] leading-5 text-gray-600 max-h-[calc(100vh_-_calc(var(--spacing)_*_28))] w-48 sticky dark:text-gray-400 pt-1",
            div { class: "border-b border-gray-300 pb-2 dark:border-[#a4a9ac7d]",
                h2 { class: "pb-2 font-semibold text-gray-600 dark:text-gray-100",
                    "On this page"
                }
                ul {
                    li {
                        class: "pb-2",
                        a {
                            class: "hover:text-sky-500 dark:hover:text-sky-400",
                            href: "#introduction",
                            "Introduction"
                        }
                    }
                    li {
                        class: "pb-2",
                        a {
                            class: "hover:text-sky-500 dark:hover:text-sky-400",
                            href: "#getting-started",
                            "Getting Started"
                        }
                    }
                    li {
                        class: "pb-2 pl-2",
                        a {
                            class: "hover:text-sky-500 dark:hover:text-sky-400",
                            href: "#prerequisites",
                            "Prerequisites"
                        }
                    }
                }
            }
            h2 { class: "py-4 ",
                a {
                    class: "hover:text-sky-500 dark:hover:text-sky-400 flex flex-row items-center gap-x-1",
                    href: "https://github.com/DioxusLabs/docsite",
                    "Edit this page"
                    // External link icon placeholder
                    span { "↗" }
                }
            }
        }
    }
}
