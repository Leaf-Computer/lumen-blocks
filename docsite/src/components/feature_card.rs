use dioxus::prelude::*;

#[component]
pub fn FeatureCard(title: String, description: String, icon: Element) -> Element {
    rsx! {
        div { class: "bg-card rounded-lg border border-border p-6 text-center",
            div { class: "flex justify-center mb-4",
                {icon}
            }
            h3 { class: "text-lg font-semibold text-foreground mb-2", "{title}" }
            p { class: "text-muted-foreground", "{description}" }
        }
    }
}
