#[component]
fn MainStatsCard(
    title: String,
    value: String,
    change: String,
    positive: bool,
    icon: Element
) -> Element {
    rsx! {
        div { class: "bg-card rounded-lg border border-border p-6",
            div { class: "flex items-center justify-between",
                div {
                    p { class: "text-sm text-muted-foreground", "{title}" }
                    p { class: "text-2xl font-bold text-foreground mt-1", "{value}" }
                    p { 
                        class: if positive { "text-green-600 text-sm mt-1" } else { "text-red-600 text-sm mt-1" },
                        "{change} from last month"
                    }
                }
                div { class: "text-primary", {icon} }
            }
        }
    }
}
