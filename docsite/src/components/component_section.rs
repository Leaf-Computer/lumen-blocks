
#[component]
fn ComponentSection(title: String, description: String, children: Element) -> Element {
    rsx! {
        div { class: "bg-card rounded-lg border border-border p-6",
            div { class: "mb-6",
                h2 { class: "text-xl font-semibold text-foreground mb-2", "{title}" }
                p { class: "text-muted-foreground", "{description}" }
            }
            {children}
        }
    }
}
