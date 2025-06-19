use dioxus::prelude::*;

#[component]
pub fn Err404(segments: Vec<String>) -> Element {
    rsx! {
        "404 not found"
    }
}
