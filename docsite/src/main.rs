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

#[derive(Clone, Routable, PartialEq, Eq, Debug)]
enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},

    #[layout(DocsLayout)]
    #[nest("/docs")]
    #[redirect("/", || Route::Docs01 { child: docs::router_01::BookRoute::Index { section: Default::default() } })]
    #[child("/0.1")]
    Docs01 { child: docs::router_01::BookRoute },
    #[end_nest]
    #[end_layout]
    #[end_layout]
    #[layout(MainLayout)]
    #[route("/404")]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

// The server function at the endpoint "static_routes" will be called by the CLI to generate the list of static
// routes. You must explicitly set the endpoint to `"static_routes"` in the server function attribute instead of
// the default randomly generated endpoint.
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

fn main() {
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
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

        // Include the analytics script (will be empty if feature is disabled)
        {analytics_script}

        div { class: "min-h-screen bg-background",
            Router::<Route> {}
        }
    }
}
