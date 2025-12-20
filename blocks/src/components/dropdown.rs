use crate::{use_id_or, use_unique_id};
use dioxus::html::GlobalAttributesExtension;
use dioxus::prelude::*;
pub use dioxus_primitives::dropdown_menu::DropdownMenuTrigger as DropdownTrigger;
use dioxus_primitives::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem};
use lucide_dioxus::Check;

// Define a context struct for radio groups
#[derive(Clone, PartialEq)]
struct RadioGroupContext<T: Clone + PartialEq + 'static> {
    value: Signal<T>,
    on_change: Callback<T>,
}

/// Dropdown size options
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DropdownSize {
    Small,
    #[default]
    Medium,
    Large,
}

// Note: DropdownSize is kept for backward compatibility but no longer used internally

// DropdownProps - Props for the main Dropdown component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    /// Whether the dropdown should be open by default
    #[props(default = false)]
    default_open: bool,

    /// Optional ID for the dropdown
    #[props(default)]
    id: Option<String>,

    /// Whether the dropdown is disabled
    #[props(default)]
    disabled: bool,

    /// Accessible label for the dropdown
    #[props(default)]
    aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

// Dropdown Trigger Props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownTriggerProps {
    /// Whether the trigger is disabled
    #[props(default)]
    disabled: bool,

    /// Optional ID for the trigger
    #[props(default)]
    id: Option<String>,

    /// Optional aria-label for the trigger (for accessibility)
    #[props(default)]
    aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

// Dropdown Content Props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownContentProps {
    /// Alignment of the dropdown menu
    #[props(default = String::from("start"))]
    align: String,

    /// Width of the dropdown menu (use class names like "w-56")
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

// Dropdown Item Props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownItemProps<T: Clone + PartialEq + 'static> {
    /// The value of the item
    value: ReadSignal<T>,

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

    /// The callback function that will be called when the item is selected. The value of the item will be passed as an argument.
    #[props(default)]
    pub on_select: Callback<T>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    // Generate unique ID if not provided
    let dropdown_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(dropdown_id, props_id.into());
    let mut is_open = use_signal::<Option<bool>>(|| Some(false));

    // Determine base classes for dropdown
    let dropdown_classes = vec![
        "relative inline-block text-left",
        if props.disabled {
            "opacity-50 pointer-events-none"
        } else {
            ""
        },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let disabled_val = props.disabled;

    rsx! {
        DropdownMenu {
            open: is_open,
            on_open_change: move |new_open| is_open.set(Some(new_open)),
            class: dropdown_classes,
            id: id_value,
            default_open: props.default_open,
            // Note: DropdownMenuTrigger doesn't have a disabled prop, using class and aria attributes instead
            "aria-disabled": if disabled_val { "true" } else { "false" },
            aria_label: props.aria_label.clone(),
            div {
                tabindex: 0,    // Make this focusable
                {props.children}
            }
        }
    }
}

#[component]
pub fn DropdownContent(props: DropdownContentProps) -> Element {
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
    let content_classes = [
        "absolute mt-2 rounded bg-popover shadow-md",
        "border border-border p-1 text-popover-foreground",
        "animate-in fade-in-80 data-[side=bottom]:slide-in-from-top-2",
        "data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2",
        "data-[side=top]:slide-in-from-bottom-2 z-50",
        align_class,
        &props.width,
    ]
    .join(" ");

    rsx! {
        DropdownMenuContent {
            class: content_classes,
            id: id_value,

            {props.children}
        }
    }
}

// Dropdown Label Props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownLabelProps {
    /// Optional ID for the label
    #[props(default)]
    id: ReadSignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

#[component]
pub fn DropdownLabel(props: DropdownLabelProps) -> Element {
    // Generate unique ID if not provided
    let label_id = use_unique_id();
    let id_value = use_id_or(label_id, props.id);

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

// Dropdown Separator Props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownSeparatorProps {
    /// Optional ID for the separator
    #[props(default)]
    id: ReadSignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn DropdownSeparator(props: DropdownSeparatorProps) -> Element {
    // Generate unique ID if not provided
    let separator_id = use_unique_id();
    let id_value = use_id_or(separator_id, props.id);

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

// Dropdown Checkbox Item Props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownCheckboxItemProps {
    /// The index of the item
    #[props(default)]
    index: ReadSignal<usize>,

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
    on_change: Option<Callback<bool>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

#[component]
pub fn DropdownCheckboxItem(props: DropdownCheckboxItemProps) -> Element {
    // Generate unique ID if not provided
    let item_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(item_id, props_id.into());

    // Handle change event
    let handle_change = move |_: bool| {
        if let Some(handler) = &props.on_change {
            handler.call(!props.checked);
        }
    };

    // Determine item classes
    let item_classes = vec![
        // Base classes
        "relative flex cursor-pointer select-none items-center rounded px-2 py-1.5",
        "text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground",
        "data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
        // State classes
        if props.disabled {
            "pointer-events-none opacity-50"
        } else {
            "hover:bg-accent hover:text-accent-foreground"
        },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        DropdownMenuItem::<bool> {
            class: item_classes,
            id: id_value,
            value: props.checked,
            index: props.index,
            disabled: props.disabled,
            on_select: handle_change,

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

// Dropdown Radio Group Props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownRadioGroupProps {
    /// The value of the selected radio item
    #[props(default)]
    value: Signal<String>,

    /// Optional ID for the radio group
    #[props(default)]
    id: ReadSignal<Option<String>>,

    /// Callback when the selection changes
    #[props(default)]
    on_value_change: Option<EventHandler<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// Children elements
    children: Element,
}

#[component]
pub fn DropdownRadioGroup(props: DropdownRadioGroupProps) -> Element {
    // Generate unique ID if not provided
    let group_id = use_unique_id();
    let id_value = use_id_or(group_id, props.id);

    // Create a context with value signal and change handler
    if let Some(handler) = &props.on_value_change {
        let context = RadioGroupContext {
            value: props.value,
            on_change: *handler,
        };
        provide_context(context);
    }

    rsx! {
        div {
            id: id_value,
            role: "radiogroup",
            class: "dropdown-radio-group",

            {props.children}
        }
    }
}

// Dropdown Radio Item Props
#[derive(Props, Clone, PartialEq)]
pub struct DropdownRadioItemProps<T: Clone + PartialEq + 'static> {
    /// The value of the radio item
    value: T,

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
pub fn DropdownRadioItem<T: Clone + PartialEq + 'static>(
    props: DropdownRadioItemProps<T>,
) -> Element {
    // Generate unique ID if not provided
    let item_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(item_id, props_id.into());

    // Get the radio group context if available
    let context = use_context::<RadioGroupContext<T>>();

    // Check if this item is selected based on context
    let is_selected: bool = *context.value.read() == props.value;

    // Determine item classes
    let item_classes = vec![
        // Base classes
        "relative flex cursor-pointer select-none items-center rounded px-2 py-1.5",
        "text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground",
        "data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
        // State classes
        if props.disabled {
            "pointer-events-none opacity-50"
        } else {
            "hover:bg-accent hover:text-accent-foreground"
        },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let handle_select = move |value: T| {
        context.on_change.call(value);
    };

    rsx! {
        DropdownMenuItem::<T> {
            class: item_classes,
            id: id_value,
            value: ReadSignal::new(Signal::new(props.value)),
            index: ReadSignal::new(Signal::new(props.index)),
            disabled: props.disabled,
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
pub fn DropdownItem<T: Clone + PartialEq + 'static>(props: DropdownItemProps<T>) -> Element {
    // Generate unique ID if not provided
    let item_id = use_unique_id();
    let props_id = use_signal(|| props.id);
    let id_value = use_id_or(item_id, props_id.into());

    // Determine item classes
    let item_classes = vec![
        // Base classes
        "relative flex cursor-pointer select-none items-center rounded px-2 py-1.5",
        "text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground",
        "data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
        "disabled:pointer-events-none disabled:opacity-50 hover:bg-secondary hover:text-accent-foreground",

        // Destructive style
        if props.destructive { "text-destructive focus:text-destructive" } else { "" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let index_val = props.index;
    let disabled_val = props.disabled;

    let icon_element = if let Some(icon) = &props.icon {
        rsx! {
            span {
                class: "mr-2",
                aria_hidden: "true",
                {icon.clone()}
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        DropdownMenuItem::<T> {
            class: item_classes,
            id: id_value,
            value: props.value,
            index: ReadSignal::<usize>::new(Signal::new(index_val)),
            disabled: disabled_val,
            on_select: props.on_select,

            {icon_element}

            {props.children}
        }
    }
}
