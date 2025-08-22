use dioxus::document;
use dioxus::prelude::*;

mod components;
mod layouts;
mod pages;
use crate::layouts::{DocsLayout, MainLayout};
use crate::pages::{Err404, Home};
use docs::docs;

const FAVICON: Asset = asset!("/assets/lumen-logo-small.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LUMEN_LOGO: Asset = asset!("/assets/lumen-logo.png");
const LUMEN_LOGO_SMALL: Asset = asset!("/assets/lumen-logo-small.png");
const LUMEN_PREVIEW_IMAGE: Asset = asset!("/assets/lumen-blocks-preview.jpg");

#[derive(Clone, Routable, PartialEq, Eq, Debug)]
enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},

    #[layout(DocsLayout)]
    #[nest("/docs")]
    #[redirect("/", || Route::Docs { child: docs::router::BookRoute::Index { section: Default::default() } })]
    #[child("/")]
    Docs { child: docs::router::BookRoute },
    #[end_nest]
    #[end_layout]
    #[end_layout]
    #[layout(MainLayout)]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Create the analytics script based on feature flag
    #[cfg(feature = "analytics")]
    let analytics_script = rsx! {
        script {
            defer: true,
            "data-domain": "lumenblocks.dev",
            src: "https://plausible.io/js/script.js"
        }
    };

    // Empty script block when analytics feature is disabled
    #[cfg(not(feature = "analytics"))]
    let analytics_script = rsx! {};

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Meta { property: "og:url", content: "https://lumenblocks.dev" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: "Lumen Blocks - Styled components for Rust Applications" }
        document::Meta { property: "og:description", content: "Lumen Blocks is a styled, opinionated UI component library for Dioxus." }
        document::Meta { property: "og:image", content: LUMEN_PREVIEW_IMAGE }
        document::Meta { name: "twitter:card", content: "summary_large_image" }

        // Include the analytics script (will be empty if feature is disabled)
        {analytics_script}

        div { class: "min-h-screen bg-background",
            Router::<Route> {}
        }
    }
}
