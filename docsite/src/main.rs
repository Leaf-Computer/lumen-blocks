use dioxus::prelude::*;
use dioxus_router::prelude::*;
use laminar_blocks::components::{
    avatar::{Avatar, AvatarImage, AvatarFallback},
    button::{Button, ButtonVariant, ButtonSize},
    input::{Input, InputSize},
    progress::{Progress, ProgressSize, ProgressVariant},
    switch::Switch,
};
use lucide_dioxus::{Check, Info, X};

mod components;
mod pages;
mod layouts;
use crate::components::{ProjectCard, StatsCard};
use crate::pages::{Err404, Dashboard, Home};
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
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LAMINAR_LOGO: Asset = asset!("/assets/laminar-logo.png");

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

#[component]
// Route handler for the Components page
fn ComponentsRoute() -> Element {
    rsx! {
        pages::Components {}
    }
}

#[component]
fn DashboardRoute() -> Element {
    rsx! {
        Dashboard {}
    }
}

pub(crate) static SHOW_SIDEBAR: GlobalSignal<bool> = Signal::global(|| false);
