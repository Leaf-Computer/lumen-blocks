use dioxus::prelude::*;

/// A reusable card component for showcasing components in the bento grid
#[component]
pub fn ComponentCard(
    // Title of the component card
    title: String,
    // Route to the component documentation
    #[props(into)]
    doc_route: NavigationTarget,
    // Optional grid column span class (e.g., "md:col-span-2")
    #[props(default)]
    col_span: Option<String>,
    // Optional grid row span class (e.g., "md:row-span-2")
    #[props(default)]
    row_span: Option<String>,
    // Optional additional classes for the content container
    #[props(default)]
    content_class: Option<String>,
    // Child elements to render inside the card
    children: Element
) -> Element {
    let grid_classes = match (col_span, row_span) {
        (Some(col), Some(row)) => format!("{} {}", col, row),
        (Some(col), None) => col,
        (None, Some(row)) => row,
        (None, None) => String::new()
    };
    
    let content_classes = content_class
        .map(|classes| format!("flex justify-stretch py-2 {}", classes))
        .unwrap_or_else(|| "flex justify-stretch py-2".to_string());

    rsx! {
        div { 
            class: "bg-background rounded-lg border border-border p-6 transition-all hover:shadow-md {grid_classes}",
            div { 
                class: "flex justify-between items-center mb-4",
                h3 { class: "font-medium text-foreground", "{title}" }
                Link { 
                    to: doc_route,
                    class: "text-sm text-primary hover:underline",
                    "View Docs"
                }
            }
            div { 
                class: "{content_classes}", 
                {children}
            }
        }
    }
}
