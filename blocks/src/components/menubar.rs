use dioxus::html::GlobalAttributesExtension;
use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    Menubar as PrimitiveMenubar, MenubarContent as PrimitiveMenubarContent,
    MenubarItem as PrimitiveMenubarItem, MenubarMenu as PrimitiveMenubarMenu,
    MenubarTrigger as PrimitiveMenubarTrigger,
};

/// Menubar main container, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct MenubarProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let default_classes =
        "flex items-center gap-1 px-1 py-1 border border-border bg-background rounded-md shadow-sm";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveMenubar {
            class: class,
            {props.children}
        }
    }
}

/// MenubarMenu: A single menu in the menubar, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct MenubarMenuProps {
    #[props(default)]
    pub class: Option<String>,
    pub index: ReadOnlySignal<usize>,
    pub children: Element,
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let default_classes = "relative group flex flex-col items-stretch";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveMenubarMenu {
            class: class,
            index: props.index,
            {props.children}
        }
    }
}

/// MenubarTrigger: The button that opens a menu, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct MenubarTriggerProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let default_classes = "px-3 py-1.5 rounded-sm text-sm font-medium text-foreground hover:bg-muted focus:outline-none focus:ring-2 focus:ring-ring transition-colors";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveMenubarTrigger {
            class: class,
            {props.children}
        }
    }
}

/// MenubarContent: The dropdown content for a menu, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct MenubarContentProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let default_classes = "hidden group-data-[state=open]:block absolute left-0 top-full mt-2 min-w-[10rem] bg-popover border border-border rounded-md shadow-lg z-50 p-1";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveMenubarContent {
            class: class,
            {props.children}
        }
    }
}

/// MenubarItem: An item in a menu, styled with Tailwind
#[derive(Props, Clone, PartialEq)]
pub struct MenubarItemProps {
    /// The index of this item within the [`MenubarContent`]. This is used to define the focus order for keyboard navigation.
    pub index: ReadOnlySignal<usize>,
    #[props(default)]
    pub class: Option<String>,
    pub value: String,
    #[props(default)]
    pub on_select: Callback<String>,
    pub children: Element,
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    let default_classes = "menubar-item cursor-pointer select-none px-3 py-1.5 rounded-sm text-sm text-foreground hover:bg-accent focus:bg-accent focus:outline-none transition-colors";
    let class = if let Some(extra) = &props.class {
        format!("{} {}", extra, default_classes)
    } else {
        default_classes.to_string()
    };

    rsx! {
        PrimitiveMenubarItem {
            index: props.index,
            class: class,
            value: props.value.clone(),
            on_select: props.on_select.clone(),
            {props.children}
        }
    }
}
