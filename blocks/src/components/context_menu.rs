use crate::{use_id_or, use_unique_id};
use dioxus_lib::prelude::*;
use dioxus_primitives::context_menu::{ContextMenu as PrimitiveContextMenu, ContextMenuContent as PrimitiveContextMenuContent, ContextMenuItem as PrimitiveContextMenuItem};

// Define a context struct for radio groups
#[derive(Clone, PartialEq)]
struct RadioGroupContext {
    value: Signal<String>,
    on_change: EventHandler<String>,
}

// ContextMenuProps - Props for the main ContextMenu component
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {
    /// Optional ID for the context menu
    #[props(default)]
    id: Option<String>,

    /// Whether the context menu is disabled
    #[props(default)]
    disabled: bool,

    /// Accessible label for the context menu
    #[props(default)]
    aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}



// ContextMenu Content Props
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    /// Alignment of the context menu
    #[props(default = String::from("start"))]
    align: String,

    /// Width of the context menu (use class names like "w-56")
    #[props(default = String::from("w-56"))]
    width: String,

    /// Optional ID for the content
    #[props(default)]
    id: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

// ContextMenu Item Props
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps {
    /// The value of the item
    #[props(default)]
    value: String,

    /// The index of the item
    #[props(default)]
    index: usize,

    /// Whether the item is disabled
    #[props(default)]
    disabled: bool,

    /// Whether the item is destructive (red)
    #[props(default)]
    destructive: bool,

    /// Optional icon to display before the item text
    #[props(default)]
    icon: Option<Element>,

    /// Optional ID for the item
    #[props(default)]
    id: Option<String>,

    /// Callback when the item is selected
    #[props(default)]
    on_select: Option<EventHandler<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    // Generate unique ID if not provided
    let context_menu_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(context_menu_id, props_id.into());

    // Determine base classes for context menu
    let context_menu_classes = vec![
        "relative", 
        if props.disabled { "opacity-50 pointer-events-none" } else { "" }
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let disabled_val = props.disabled;

    rsx! {
        PrimitiveContextMenu {
            class: context_menu_classes,
            id: id_value,
            "aria-disabled": if disabled_val { "true" } else { "false" },
            aria_label: props.aria_label.clone(),

            {props.children}
        }
    }
}

pub use dioxus_primitives::context_menu::ContextMenuTrigger;
use lucide_dioxus::Check;

#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    // Generate unique ID if not provided
    let content_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(content_id, props_id.into());

    // Alignment classes
    let align_class = match props.align.as_str() {
        "end" => "right-0 origin-top-right",
        "center" => "left-1/2 -translate-x-1/2 origin-top",
        _ => "left-0 origin-top-left", // Default to start
    };

    // Content classes
    let content_classes = vec![
        "absolute mt-1 rounded-md bg-popover shadow-lg",
        "border border-border p-1 text-popover-foreground",
        "animate-in fade-in-80 data-[side=bottom]:slide-in-from-top-2",
        "data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2",
        "data-[side=top]:slide-in-from-bottom-2 z-50",
        align_class,
        &props.width,
    ]
    .join(" ");

    rsx! {
        PrimitiveContextMenuContent {
            class: content_classes,
            id: id_value,

            {props.children}
        }
    }
}

// ContextMenu Label Props
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuLabelProps {
    /// Optional ID for the label
    #[props(default)]
    id: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

#[component]
pub fn ContextMenuLabel(props: ContextMenuLabelProps) -> Element {
    // Generate unique ID if not provided
    let label_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(label_id, props_id.into());

    // Label classes
    let label_classes = "px-2 py-1.5 text-xs font-semibold text-foreground/80";

    rsx! {
        div {
            class: label_classes,
            id: id_value,
            ..props.attributes,
            {props.children}
        }
    }
}

// ContextMenu Separator Props
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuSeparatorProps {
    /// Optional ID for the separator
    #[props(default)]
    id: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn ContextMenuSeparator(props: ContextMenuSeparatorProps) -> Element {
    // Generate unique ID if not provided
    let separator_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(separator_id, props_id.into());

    // Separator classes
    let separator_classes = "h-px my-1 bg-muted";

    rsx! {
        div {
            class: separator_classes,
            id: id_value,
            role: "separator",
            aria_orientation: "horizontal",
            ..props.attributes,
        }
    }
}

// ContextMenu Checkbox Item Props
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuCheckboxItemProps {
    /// The value of the item
    #[props(default)]
    value: String,

    /// The index of the item
    #[props(default)]
    index: usize,

    /// Whether the checkbox is checked
    #[props(default)]
    checked: bool,

    /// Whether the item is disabled
    #[props(default)]
    disabled: bool,

    /// Optional ID for the item
    #[props(default)]
    id: Option<String>,

    /// Callback when the item is selected
    #[props(default)]
    on_change: Option<EventHandler<bool>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

#[component]
pub fn ContextMenuCheckboxItem(props: ContextMenuCheckboxItemProps) -> Element {
    // Generate unique ID if not provided
    let item_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(item_id, props_id.into());

    // Handle change event
    let handle_change = move || {
        if let Some(handler) = &props.on_change {
            handler.call(!props.checked);
        }
    };

    // Determine item classes
    let item_classes = vec![
        // Base classes
        "relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5",
        "text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground",
        "data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
        
        // State classes
        if props.disabled { "pointer-events-none opacity-50" } else { "hover:bg-accent hover:text-accent-foreground" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let value_str = props.value;
    let index_val = props.index;

    rsx! {
        PrimitiveContextMenuItem {
            class: item_classes,
            id: id_value,
            value: ReadOnlySignal::new(Signal::new(value_str)),
            index: ReadOnlySignal::new(Signal::new(index_val)),
            on_select: move |_| handle_change(),
            
            // Checkbox indicator
            span {
                class: "mr-2 h-4 w-4 flex items-center justify-center border-none",
                aria_hidden: "true",

                if props.checked {
                    Check {
                        class: "h-4 w-4 text-current",
                    }
                }
            }

            {props.children}
        }
    }
}

// ContextMenu Radio Group Props
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuRadioGroupProps {
    /// The value of the selected radio item
    #[props(default)]
    value: Signal<String>,

    /// Optional ID for the radio group
    #[props(default)]
    id: Option<String>,

    /// Callback when the selection changes
    #[props(default)]
    on_value_change: Option<EventHandler<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

#[component]
pub fn ContextMenuRadioGroup(props: ContextMenuRadioGroupProps) -> Element {
    // Generate unique ID if not provided
    let group_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(group_id, props_id.into());
    
    // Create a context with value signal and change handler
    if let Some(handler) = &props.on_value_change {
        let context = RadioGroupContext {
            value: props.value,
            on_change: handler.clone(),
        };
        provide_context(context);
    }

    rsx! {
        div {
            id: id_value,
            role: "radiogroup",
            class: "context-menu-radio-group",

            {props.children}
        }
    }
}

// ContextMenu Radio Item Props
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuRadioItemProps {
    /// The value of the radio item
    #[props(default)]
    value: String,

    /// The index of the item
    #[props(default)]
    index: usize,

    /// Whether the item is disabled
    #[props(default)]
    disabled: bool,

    /// Optional ID for the item
    #[props(default)]
    id: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

#[component]
pub fn ContextMenuRadioItem(props: ContextMenuRadioItemProps) -> Element {
    // Generate unique ID if not provided
    let item_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(item_id, props_id.into());

    // Get the radio group context if available
    let context = use_context::<RadioGroupContext>();
    
    // Check if this item is selected based on context
    let is_selected = *context.value.read() == props.value;

    // Determine item classes
    let item_classes = vec![
        // Base classes
        "relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5",
        "text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground",
        "data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
        
        // State classes
        if props.disabled { "pointer-events-none opacity-50" } else { "hover:bg-accent hover:text-accent-foreground" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let value_str = props.value;
    let index_val = props.index;

    // When selected, call the group's value change handler
    let value_for_handler = value_str.clone();
    let context_clone = context.clone();
    let handle_select = move |_| {
        context_clone.on_change.call(value_for_handler.clone());
    };

    rsx! {
        PrimitiveContextMenuItem {
            class: item_classes,
            id: id_value,
            value: value_str,
            index: index_val,
            on_select: handle_select,

            // Radio indicator
            span {
                class: "mr-2 h-3.5 w-3.5 flex items-center justify-center rounded-full border",
                aria_hidden: "true",
                
                // The dot will be shown when this item is selected
                span {
                    class: "h-1.5 w-1.5 rounded-full bg-current",
                    style: if is_selected { "opacity: 1" } else { "opacity: 0" },
                }
            }

            {props.children}
        }
    }
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    // Generate unique ID if not provided
    let item_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(item_id, props_id.into());

    // Determine item classes
    let item_classes = vec![
        // Base classes
        "relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5",
        "text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground",
        "data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
        "disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground",
        
        // Destructive style
        if props.destructive { "text-destructive focus:text-destructive" } else { "" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let value_str = props.value;
    let index_val = props.index;
    
    // Handle select event - clone value early to avoid move issues
    let value_for_handler = value_str.clone();
    let handler_clone = props.on_select.clone();
    let handle_select = move |_| {
        if let Some(handler) = &handler_clone {
            handler.call(value_for_handler.clone());
        }
    };

    rsx! {
        PrimitiveContextMenuItem {
            class: item_classes,
            id: id_value,
            value: value_str,
            index: index_val,
            on_select: handle_select,

            // Icon if provided
            if let Some(icon) = &props.icon {
                span {
                    class: "mr-2",
                    aria_hidden: "true",
                    {icon.clone()}
                }
            }

            {props.children}
        }
    }
}
