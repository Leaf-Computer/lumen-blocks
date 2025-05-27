use dioxus::prelude::*;
use dioxus_primitives::toast::*;
use dioxus_primitives::collapsible::*;
use dioxus_primitives::separator::*;
// Import examples as modules
mod button_example;
use button_example::ButtonExample;
mod form_example;
use form_example::FormExample;

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
            
            Collapsible {
                CollapsibleTrigger { "Form Example" }
                CollapsibleContent { FormExample {} }
            }
        }
    }
}
