use crate::use_unique_id;
use dioxus_lib::html::GlobalAttributesExtension;
use dioxus_lib::prelude::*;
use dioxus_primitives::aspect_ratio::AspectRatio as PrimitiveAspectRatio;

/// Props for the AspectRatio component
#[derive(Props, Clone, PartialEq)]
pub struct AspectRatioProps {
    /// The aspect ratio (width / height)
    pub ratio: f64,

    /// Optional ID for the aspect ratio container
    #[props(default)]
    pub id: Option<String>,

    /// Optional class name for additional styling
    #[props(default)]
    pub class: Option<String>,

    /// Children to render inside the aspect ratio container
    pub children: Element,
}

/// A styled aspect ratio container that maintains consistent proportions
#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    // Generate unique ID if not provided
    let aspect_ratio_id = use_unique_id();
    let id_value = use_memo(move || {
        props
            .id
            .clone()
            .unwrap_or_else(|| aspect_ratio_id.peek().clone())
    });

    // Build classes - combining default styling with optional custom classes
    let full_classes = vec![
        // Base classes
        "relative w-full overflow-hidden",
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveAspectRatio {
            id: id_value.peek().clone(),
            class: full_classes,
            ratio: props.ratio,

            {props.children}
        }
    }
}
