use dioxus::prelude::*;
use dioxus::document;

mod components;
mod pages;
mod layouts;
use crate::pages::{Err404, Home};
use crate::layouts::{DocsLayout, MainLayout};
use docs::docs;

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
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/laminar-logo-small.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LAMINAR_LOGO: Asset = asset!("/assets/laminar-logo.png");
const LAMINAR_LOGO_SMALL: Asset = asset!("/assets/laminar-logo-small.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen bg-background",
            Router::<Route> {}
        }
    }
}
