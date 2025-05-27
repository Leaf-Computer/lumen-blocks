use dioxus::prelude::*;
use dioxus_primitives::toast::*;
use dioxus_primitives::collapsible::*;
use dioxus_primitives::separator::*;
// Import ButtonExample as a module
mod button_example;
use button_example::ButtonExample;

const TAILWIND_CSS: Asset = asset!("assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        ToastProvider {
            Collapsible {
                CollapsibleTrigger { "Button Example" }
                CollapsibleContent { ButtonExample {} }
            }
    
            Separator {
                class: "separator",
                style: "margin: 15px 0;",
                horizontal: true,
                decorative: true,
            }
        }
    }
}
