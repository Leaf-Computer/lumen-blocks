use dioxus::prelude::*;
use dioxus_blocks::components::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleTrigger,
};
use dioxus_primitives::separator::*;
use std::time::Duration;
// Import examples as modules
mod button_example;
use button_example::ButtonExample;
mod form_example;
use form_example::FormExample;
mod dropdown_example;
use dropdown_example::DropdownExample;
mod context_menu_example;
use context_menu_example::ContextMenuExample;
mod switch_example;
use switch_example::SwitchExample;
mod menubar_example;
use menubar_example::MenubarExample;
mod accordion_example;
use accordion_example::accordion_examples;
mod hover_card_example;
use hover_card_example::HoverCardExample;
mod checkbox_example;
use checkbox_example::CheckboxExample;
mod avatar_example;
use avatar_example::avatar_examples;
mod collapsible_example;
use collapsible_example::CollapsibleExample;
mod aspect_ratio_example;
use aspect_ratio_example::AspectRatioExample;
use dioxus_blocks::components::toast::ToastProvider;
mod toast_example;
use toast_example::ToastExamples;
mod progress_example;
use progress_example::ProgressExample;

const TAILWIND_CSS: Asset = asset!("assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        ToastProvider {
            default_duration: Duration::from_secs(5),
            div { class: "p-6 flex flex-col gap-4",
                Collapsible {
                    CollapsibleTrigger { "Button Example" }
                    CollapsibleContent { ButtonExample {} }
                }
    
                Collapsible {
                    CollapsibleTrigger { "Form Example" }
                    CollapsibleContent { FormExample {} }
                }
    
                Collapsible {
                    CollapsibleTrigger { "Dropdown Example" }
                    CollapsibleContent { DropdownExample {} }
                }
    
                Collapsible {
                    CollapsibleTrigger { "Context Menu Example" }
                    CollapsibleContent { ContextMenuExample {} }
                }
    
                Collapsible {
                    CollapsibleTrigger { "Menubar Example" }
                    CollapsibleContent { MenubarExample {} }
                }
    
                Collapsible {
                    CollapsibleTrigger { "Switch Example" }
                    CollapsibleContent { SwitchExample {} }
                }
                
                Collapsible {
                    CollapsibleTrigger { "Accordion Example" }
                    CollapsibleContent { accordion_examples {} }
                }
                
                Collapsible {
                    CollapsibleTrigger { "Collapsible Example" }
                    CollapsibleContent { CollapsibleExample {} }
                }
    
                Collapsible {
                    CollapsibleTrigger { "Hover Card Example" }
                    CollapsibleContent { HoverCardExample {} }
                }
                
                Collapsible {
                    CollapsibleTrigger { "Checkbox Example" }
                    CollapsibleContent { CheckboxExample {} }
                }
                
                Collapsible {
                    CollapsibleTrigger { "Avatar Example" }
                    CollapsibleContent { avatar_examples {} }
                }
                
                Collapsible {
                    CollapsibleTrigger { "Aspect Ratio Example" }
                    CollapsibleContent { AspectRatioExample {} }
                }
                
                Collapsible {
                    CollapsibleTrigger { "Toast Example" }
                    CollapsibleContent { ToastExamples {} }
                }
                
                Collapsible {
                    CollapsibleTrigger { "Progress Example" }
                    CollapsibleContent { ProgressExample {} }
                }
            }
        }
    }
}
