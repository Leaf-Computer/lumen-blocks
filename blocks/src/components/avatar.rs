use crate::use_unique_id;
use dioxus_lib::prelude::*;
use dioxus_primitives::avatar::{
    Avatar as PrimitiveAvatar, AvatarFallback as PrimitiveAvatarFallback,
    AvatarImage as PrimitiveAvatarImage, AvatarState,
};

/// Props for the Avatar component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Optional additional classes for the avatar
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the avatar element
    #[props(default)]
    pub id: Option<String>,

    /// Optional callback when the avatar state changes
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    /// Child elements
    pub children: Element,
}

/// Styled wrapper for the Avatar component
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    // Generate unique ID if not provided
    let avatar_id = use_unique_id();
    let id_value = use_memo(move || {
        props.id.clone().unwrap_or_else(|| avatar_id.peek().clone())
    });

    let avatar_classes = vec![
        // Base classes - circular avatar with consistent sizing
        "relative inline-flex h-10 w-10 shrink-0 overflow-hidden rounded-full border border-border bg-muted group",
        
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let on_state_change = props.on_state_change.clone();

    rsx! {
        PrimitiveAvatar {
            id: id_value.peek().clone(),
            class: avatar_classes,
            on_state_change: move |state| {
                if let Some(handler) = &on_state_change {
                    handler.call(state);
                }
            },
            
            {props.children}
        }
    }
}

/// Props for the AvatarImage component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    /// The source URL of the image
    pub src: String,

    /// Alt text for the image
    pub alt: String,

    /// Optional additional classes for the image
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the image element
    #[props(default)]
    pub id: Option<String>,
}

/// Styled wrapper for the AvatarImage component
#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    // Generate unique ID if not provided
    let image_id = use_unique_id();
    let id_value = use_memo(move || {
        props.id.clone().unwrap_or_else(|| image_id.peek().clone())
    });

    let image_classes = vec![
        // Base classes - fill the container and maintain aspect ratio
        "aspect-square h-full w-full object-cover group-data-[state=error]:hidden",
        
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveAvatarImage {
            id: id_value.peek().clone(),
            class: image_classes,
            src: props.src,
            alt: props.alt,
        }
    }
}

/// Props for the AvatarFallback component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    /// Optional additional classes for the fallback
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for the fallback element
    #[props(default)]
    pub id: Option<String>,

    /// Child elements (typically text or icon)
    pub children: Element,
}

/// Styled wrapper for the AvatarFallback component
#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    // Generate unique ID if not provided
    let fallback_id = use_unique_id();
    let id_value = use_memo(move || {
        props.id.clone().unwrap_or_else(|| fallback_id.peek().clone())
    });

    let fallback_classes = vec![
        // Base classes - center content and style text
        "flex h-full w-full items-center justify-center rounded-full bg-muted text-sm font-medium text-muted-foreground",
        
        // Additional classes passed by the user
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        PrimitiveAvatarFallback {
            id: id_value.peek().clone(),
            class: fallback_classes,
            
            {props.children}
        }
    }
}
