use crate::doc_examples::*;
use dioxus::prelude::*;
use laminar_blocks::components::button::{Button, ButtonSize, ButtonVariant};
use std::hash::Hash;
use lucide_dioxus::{Check, Copy};

pub mod router_01;

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
            class: "bg-background border border-border rounded-lg shadow p-6 my-6 overflow-auto text-foreground",
            {children}
        }
    }
}

fn LayoutsExplanation() -> Element {
    rsx! {
        pre {
            class: "p-4 bg-card rounded-lg text-sm font-mono text-card-foreground overflow-auto",
            onmouseenter: move |_| {},
            onmouseleave: move |_| {},
            span {
                "#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {{\n\t"
            }
            span {
                onmouseenter: move |_| {},
                onmouseleave: move |_| {},
                class: "border border-orange-500 rounded-md px-1",
                "#[layout(HeaderFooter)]"
            }
            span { "\n\t\t// ... other routes\n\t\t" }
            span {
                onmouseenter: move |_| {},
                onmouseleave: move |_| {},
                class: "border border-green-500 rounded-md px-1",
                r##"#[layout(DocsSidebars)]"##
            }
            "\n\t\t\t"
            span {
                onmouseenter: move |_| {},
                onmouseleave: move |_| {},
                class: "border border-blue-500 rounded-md px-1",
                r##"#[route("/learn")]"##
            }
            span { "\n\t\t\tDocs {{}},\n}}" }
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
