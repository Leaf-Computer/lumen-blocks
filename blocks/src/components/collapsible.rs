use crate::use_unique_id;
use dioxus_lib::html::GlobalAttributesExtension;
use dioxus_lib::prelude::*;
use dioxus_primitives::collapsible::{
    Collapsible as PrimitiveCollapsible, CollapsibleContent as PrimitiveCollapsibleContent,
    CollapsibleTrigger as PrimitiveCollapsibleTrigger,
};
use lucide_dioxus::ChevronDown;

/// Props for the Collapsible component
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
    /// Optional additional classes for the collapsible
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the collapsible element
    #[props(default)]
    pub id: Option<String>,

    /// Child elements
    pub children: Element,
}

/// Styled wrapper for the Collapsible component
#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    // Generate unique ID if not provided
    let collapsible_id = use_unique_id();
    let id_value = use_memo(move || {
        props
            .id
            .clone()
            .unwrap_or_else(|| collapsible_id.peek().clone())
    });

    let collapsible_classes = vec![
        // Base classes - clean container styling
        "w-full border border-border rounded-lg",
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveCollapsible {
            id: id_value.peek().clone(),
            class: collapsible_classes,
            {props.children}
        }
    }
}

/// Props for the CollapsibleTrigger component
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleTriggerProps {
    /// Optional additional classes for the trigger
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the trigger element
    #[props(default)]
    pub id: Option<String>,

    /// Child elements
    pub children: Element,
}

/// Styled wrapper for the CollapsibleTrigger component
#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    // Generate unique ID if not provided
    let trigger_id = use_unique_id();
    let id_value = use_memo(move || {
        props
            .id
            .clone()
            .unwrap_or_else(|| trigger_id.peek().clone())
    });

    let trigger_classes = vec![
        // Base classes
        "flex w-full items-center justify-between py-4 px-4 font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180",

        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveCollapsibleTrigger {
            id: id_value.peek().clone(),
            class: trigger_classes,

            {props.children}

            // Chevron icon
            ChevronDown {
                class: "h-4 w-4 shrink-0 transition-transform duration-200 data-[state=open]:rotate-180"
            }
        }
    }
}

/// Props for the CollapsibleContent component
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleContentProps {
    /// Optional additional classes for the content
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the content element
    #[props(default)]
    pub id: Option<String>,

    /// Child elements
    pub children: Element,
}

/// Styled wrapper for the CollapsibleContent component
#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    // Generate unique ID if not provided
    let content_id = use_unique_id();
    let id_value = use_memo(move || {
        props
            .id
            .clone()
            .unwrap_or_else(|| content_id.peek().clone())
    });

    let content_classes = vec![
        // Base classes - shadcn ui inspired content styling
        "overflow-hidden text-sm px-4 data-[state=closed]:animate-collapsible-up data-[state=open]:animate-collapsible-down",

        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveCollapsibleContent {
            id: id_value.peek().clone(),
            class: content_classes,
            div {
                class: "pb-4 pt-0",
                {props.children}
            }
        }
    }
}
