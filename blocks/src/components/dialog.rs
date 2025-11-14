use dioxus::prelude::*;
use dioxus_primitives::dialog;

/// The props for the [`DialogRoot`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogRootProps {
    /// The ID of the dialog root element.
    pub id: ReadSignal<Option<String>>,

    /// Extra classes
    pub class: Option<String>,

    /// Whether the dialog is modal. If true, it will trap focus within the dialog when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the dialog.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the dialog if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// A callback that is called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes to apply to the dialog root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dialog root component.
    pub children: Element,
}

#[component]
pub fn DialogRoot(props: DialogRootProps) -> Element {
    let class = [
        "fixed inset-0 z-1000 flex items-center justify-center backdrop-blur-xs bg-black/40",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        dialog::DialogRoot {
            id: props.id,
            class,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DialogRoot`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogContentProps {
    /// The ID of the dialog content element.
    pub id: ReadSignal<Option<String>>,
    /// Extra classes
    #[props(default)]
    pub class: Option<String>,
    /// Additional attributes to apply to the dialog content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dialog content.
    pub children: Element,
}

#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let class = [
        "flex flex-col border-border border-1 rounded-xl p-6 bg-background",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        dialog::DialogContent { id: props.id, class, attributes: props.attributes, {props.children} }
    }
}

/// The props for the [`DialogTitle`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    /// The ID of the dialog title element.
    pub id: ReadSignal<Option<String>>,
    /// Extra classes
    pub class: Option<String>,
    /// Additional attributes for the dialog title element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dialog title.
    pub children: Element,
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let class = [
        "text-lg font-semibold leading-none tracking-tight text-center",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        dialog::DialogTitle { id: props.id, class, attributes: props.attributes, {props.children} }
    }
}

/// The props for the [`DialogDescription`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
    /// The ID of the dialog description element.
    pub id: ReadSignal<Option<String>>,
    /// Extra classes
    pub class: Option<String>,
    /// Additional attributes for the dialog description element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dialog description.
    pub children: Element,
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let class = [
        "text-sm text-muted-foreground text-center",
        props.class.as_deref().unwrap_or(""),
    ]
    .into_iter()
    .filter(|x| !x.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    rsx! {
        dialog::DialogDescription { id: props.id, class, attributes: props.attributes, {props.children} }
    }
}
