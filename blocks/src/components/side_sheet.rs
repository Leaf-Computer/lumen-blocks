use dioxus_lib::prelude::*;
use lucide_dioxus::X;

// Side from which the sheet appears
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideSheetSide {
    Left,
    Right,
}

impl SideSheetSide {
    fn content_classes(&self) -> &'static str {
        match self {
            SideSheetSide::Left => "inset-y-0 left-0 h-full w-3/4 sm:max-w-sm",
            SideSheetSide::Right => "inset-y-0 right-0 h-full w-3/4 sm:max-w-sm",
        }
    }

    fn animation_classes(&self, is_open: bool) -> &'static str {
        match (self, is_open) {
            (SideSheetSide::Left, true) => "translate-x-0",
            (SideSheetSide::Left, false) => "-translate-x-full",
            (SideSheetSide::Right, true) => "translate-x-0",
            (SideSheetSide::Right, false) => "translate-x-full",
        }
    }
}

// Context for sharing state between side sheet components
#[derive(Clone)]
struct SideSheetContext {
    is_open: Signal<bool>,
    side: SideSheetSide,
}

// Main SideSheet component that provides context
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetProps {
    #[props(default = SideSheetSide::Right)]
    pub side: SideSheetSide,
    
    #[props(default = false)]
    pub default_open: bool,
    
    pub children: Element,
}

#[component]
pub fn SideSheet(props: SideSheetProps) -> Element {
    let is_open = use_signal(|| props.default_open);
    
    let context = SideSheetContext {
        is_open,
        side: props.side,
    };
    
    use_context_provider(|| context);
    
    rsx! {
        {props.children}
    }
}

// Trigger component to open the side sheet
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetTriggerProps {
    pub children: Element,
}

#[component]
pub fn SideSheetTrigger(props: SideSheetTriggerProps) -> Element {
    let mut context = use_context::<SideSheetContext>();
    
    let on_click = move |_| {
        context.is_open.set(true);
    };
    
    rsx! {
        div { class: "w-auto inline-block",
            onclick: on_click,
            {props.children}
        }
    }
}

// Close trigger component
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetCloseProps {
    pub children: Element,
}

#[component]
pub fn SideSheetClose(props: SideSheetCloseProps) -> Element {
    let mut context = use_context::<SideSheetContext>();
    
    let on_click = move |_| {
        context.is_open.set(false);
    };
    
    rsx! {
        div {
            onclick: on_click,
            {props.children}
        }
    }
}

// Overlay component
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetOverlayProps {
    #[props(default = "bg-black/80".to_string())]
    pub class: String,
}

#[component]
pub fn SideSheetOverlay(props: SideSheetOverlayProps) -> Element {
    let mut context = use_context::<SideSheetContext>();
    
    let on_click = move |_| {
        context.is_open.set(false);
    };
    
    let is_open = *context.is_open.read();
    
    rsx! {
        div {
            class: "fixed inset-0 z-50 {props.class} data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:hidden",
            "data-state": if is_open { "open" } else { "closed" },
            onclick: on_click,
            aria_hidden: "true",
        }
    }
}

// Content component
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetContentProps {
    #[props(default = "".to_string())]
    pub class: String,
    
    pub children: Element,
}

#[component]
pub fn SideSheetContent(props: SideSheetContentProps) -> Element {
    let context = use_context::<SideSheetContext>();
    let is_open = *context.is_open.read();
    
    // We'll handle escape key with the aria-modal attribute
    // which provides built-in keyboard handling
    
    let side_classes = context.side.content_classes();
    let animation_classes = context.side.animation_classes(is_open);
    
    rsx! {
        // Portal-like behavior - render at the root level
        div {
            class: "fixed z-50",
            
            // Overlay
            SideSheetOverlay {}
            
            // Content
            div {
                class: "fixed z-50 bg-background border-l border-border shadow-lg transition ease-in-out duration-300 {side_classes} {animation_classes} {props.class}",
                role: "dialog",
                aria_modal: "true",
                aria_labelledby: "side-sheet-title",
                aria_describedby: "side-sheet-description",
                
                // Focus trap would be implemented here in a production version
                
                {props.children}
            }
        }
    }
}

// Header component for common pattern
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetHeaderProps {
    #[props(default = "".to_string())]
    pub class: String,
    
    pub children: Element,
}

#[component]
pub fn SideSheetHeader(props: SideSheetHeaderProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col space-y-2 text-center sm:text-left {props.class}",
            {props.children}
        }
    }
}

// Title component for common pattern
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetTitleProps {
    #[props(default = "".to_string())]
    pub class: String,
    
    pub children: Element,
}

#[component]
pub fn SideSheetTitle(props: SideSheetTitleProps) -> Element {
    rsx! {
        h2 {
            id: "side-sheet-title",
            class: "text-lg font-semibold leading-none tracking-tight {props.class}",
            {props.children}
        }
    }
}

// Description component for common pattern
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetDescriptionProps {
    #[props(default = "".to_string())]
    pub class: String,
    
    pub children: Element,
}

#[component]
pub fn SideSheetDescription(props: SideSheetDescriptionProps) -> Element {
    rsx! {
        p {
            id: "side-sheet-description",
            class: "text-sm text-muted-foreground {props.class}",
            {props.children}
        }
    }
}

// Body component for main content area
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetBodyProps {
    #[props(default = "".to_string())]
    pub class: String,
    
    pub children: Element,
}

#[component]
pub fn SideSheetBody(props: SideSheetBodyProps) -> Element {
    rsx! {
        div {
            class: "flex-1 overflow-y-auto {props.class}",
            {props.children}
        }
    }
}

// Footer component for action buttons
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetFooterProps {
    #[props(default = "".to_string())]
    pub class: String,
    
    pub children: Element,
}

#[component]
pub fn SideSheetFooter(props: SideSheetFooterProps) -> Element {
    rsx! {
        div {
            class: "flex gap-2 {props.class}",
            {props.children}
        }
    }
}

// Default close button component for convenience
#[derive(Props, Clone, PartialEq)]
pub struct SideSheetCloseButtonProps {
    #[props(default = "".to_string())]
    pub class: String,
}

#[component]
pub fn SideSheetCloseButton(props: SideSheetCloseButtonProps) -> Element {
    let mut context = use_context::<SideSheetContext>();
    
    rsx! {
        button {
            class: "absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-secondary {props.class}",
            onclick: move |_| context.is_open.set(false),
            type: "button",
            aria_label: "Close",
            
            X { 
                class: "h-6 w-6" 
            }
        }
    }
}
