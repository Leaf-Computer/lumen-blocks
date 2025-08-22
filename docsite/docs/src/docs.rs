pub use crate::doc_examples::*;
use dioxus::prelude::*;
use lucide_dioxus::{Check, Copy};
use lumen_blocks::components::button::{Button, ButtonSize, ButtonVariant};
use std::hash::Hash;

pub mod router;

#[component]
fn SandBoxFrame(url: String) -> Element {
    rsx! {
        iframe {
            class: "border rounded-md border-border shadow-sm",
            width: "800",
            height: "450",
            src: "{url}?embed=1",
            "allowfullscreen": true,
        }
    }
}

#[component]
fn DemoFrame(children: Element) -> Element {
    rsx! {
        div {
            class: "bg-background border border-border rounded-lg shadow p-6 my-6 overflow-visible text-foreground",
            {children}
        }
    }
}

#[component]
fn CodeBlock(contents: String, name: Option<String>) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        div {
            class: "rounded-lg border border-border shadow-sm mb-6 overflow-hidden",
            div {
                class: "bg-card flex justify-between items-center p-2 text-xs font-mono rounded-t-lg",
                div {
                    class: "text-card-foreground",
                    if let Some(path) = name {
                        "src/{path}"
                    }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Small,
                    on_click: move |_| { copied.set(true); },
                    "onclick": "navigator.clipboard.writeText(this.parentNode.parentNode.lastChild.innerText);",
                    if copied() {
                        div { class: "flex gap-1 text-green-500 items-center",
                            Check { class: "w-4 h-4" }
                            "Copied!"
                        }
                    }
                    else {
                        Copy { class: "w-4 h-4" }
                    }
                }
            }
            div { class: "codeblock text-xs bg-[#0d0d0d] p-4 overflow-auto", dangerous_inner_html: contents }
        }
    }
}
