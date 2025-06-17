use dioxus_lib::prelude::*;
use dioxus_primitives::hover_card::{
    HoverCard as PrimitiveHoverCard, HoverCardContent as PrimitiveHoverCardContent, HoverCardTrigger as PrimitiveHoverCardTrigger,
};

pub use dioxus_primitives::hover_card::{HoverCardAlign, HoverCardSide};

/// HoverCard main container, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let default_classes = "relative inline-block";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveHoverCard {
            class: class,
            {props.children}
        }
    }
}

/// HoverCardTrigger: The element that triggers the hover card, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardTriggerProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    let default_classes = "cursor-pointer focus:outline-none";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveHoverCardTrigger {
            class: class,
            {props.children}
        }
    }
}

/// HoverCardContent: The floating card content, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct HoverCardContentProps {
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub side: Option<HoverCardSide>,
    #[props(default)]
    pub align: Option<HoverCardAlign>,
    pub children: Element,
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    let default_classes = "pointer-events-none opacity-0 data-[state=open]:pointer-events-auto data-[state=open]:opacity-100 absolute top-full z-50 transition-all duration-200 py-2";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveHoverCardContent {
            class: class,
            side: props.side.unwrap_or(HoverCardSide::Top),
            align: props.align.unwrap_or(HoverCardAlign::Center),
            div {
                class: "min-w-[16rem] max-w-[22rem] bg-popover border border-border rounded-xl shadow-xl p-5",
                {props.children}
            }
        }
    }
}
