use crate::components::button::{Button, ButtonVariant};
use dioxus::html::GlobalAttributesExtension;
use dioxus::prelude::*;
use dioxus_sdk_time::use_timeout;
use lucide_dioxus::{Check, Info, TriangleAlert, X};
use std::time::Duration;

// Toast types for different visual styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastType {
    fn icon_component(&self) -> Element {
        match self {
            ToastType::Success => rsx! { Check { class: "size-5" } },
            ToastType::Error => rsx! { X { class: "size-5" } },
            ToastType::Warning => rsx! { TriangleAlert { class: "size-5" } },
            ToastType::Info => rsx! { Info { class: "size-5" } },
        }
    }

    fn classes(&self) -> &'static str {
        "border-border bg-popover text-foreground"
    }

    fn icon_classes(&self) -> &'static str {
        match self {
            ToastType::Success => "text-green-600 dark:text-green-400",
            ToastType::Error => "text-red-600 dark:text-red-400",
            ToastType::Warning => "text-yellow-600 dark:text-yellow-400",
            ToastType::Info => "text-foreground",
        }
    }
}

// A single toast item
#[derive(Debug, Clone, PartialEq)]
pub struct ToastItem {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub toast_type: ToastType,
    pub duration: Option<Duration>,
    pub permanent: bool,
    pub visible: bool,
}

// Global signal for toast management
static TOASTS: GlobalSignal<Vec<ToastItem>> = Signal::global(|| Vec::new());
static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

// Toast provider props
#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    #[props(default = Duration::from_secs(5))]
    pub default_duration: Duration,

    #[props(default = 10)]
    pub max_toasts: usize,

    pub children: Element,
}

// Toast provider component
#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    rsx! {
        // Render children
        {props.children}

        // Toast container - fixed position overlay
        div {
            class: "fixed top-4 right-0 z-50 flex flex-col space-y-2 w-full max-w-sm px-4 items-end pointer-events-none",
            aria_live: "polite",
            aria_atomic: true,

            for toast in TOASTS.read().iter() {
                Toast {
                    key: "{toast.id}",
                    toast: toast.clone(),
                    default_duration: props.default_duration,
                }
            }
        }
    }
}

// Toast props
#[derive(Props, Clone, PartialEq)]
pub struct ToastProps {
    pub toast: ToastItem,
    pub default_duration: Duration,
}

// Toast component
#[component]
pub fn Toast(props: ToastProps) -> Element {
    let toast = props.toast.clone();
    let id = toast.id;
    let mut visible = use_signal(|| true);

    // Handle removing toast from the list
    let remove_toast = move || {
        let mut toasts = TOASTS.write();
        if let Some(pos) = toasts.iter().position(|t| t.id == id) {
            toasts.remove(pos);
        }
    };

    // Handle starting exit animation
    let start_exit = move |_| {
        visible.with_mut(|val| *val = false);
    };

    // Set up auto-dismiss timer if not permanent
    if !toast.permanent {
        let duration = toast.duration.unwrap_or(props.default_duration);
        let mut visible_state = visible.clone();

        // Simple timeout effect
        use_effect(move || {
            let timer = use_timeout(duration, move |()| {
                visible_state.with_mut(|val| *val = false);
            });
            timer.action(());
            // Explicitly return unit type
            ()
        });
    }

    // Base styling
    let base_classes = "pointer-events-auto relative flex w-full items-center justify-between space-x-4 overflow-hidden rounded-md border p-4 shadow-md hover:shadow-lg transition-all duration-300 group backdrop-blur-sm";

    // Animation classes based on state
    let animation_classes = if !*visible.read() {
        "animate-slide-out-to-right"
    } else {
        "animate-slide-in-from-right"
    };

    // Toast type specific classes
    let type_classes = toast.toast_type.classes();

    // Combined classes
    let combined_classes = format!("{} {} {}", base_classes, animation_classes, type_classes);

    rsx! {
        div {
            role: "alert",
            class: "{combined_classes}",
            tabindex: "0",  // Make focusable
            aria_labelledby: format!("toast-title-{}", toast.id),
            aria_describedby: if toast.description.is_some() {
                Some(format!("toast-desc-{}", toast.id))
            } else {
                None
            },
            // Simplified: no pause-on-hover for initial implementation
            onanimationend: move |_| {
                // If toast is not visible, remove it when animation ends
                if !*visible.read() {
                    remove_toast();
                }
            },

            div {
                class: "flex items-center space-x-3 flex-1",

                div {
                    class: "flex-shrink-0 {toast.toast_type.icon_classes()}",
                    aria_label: match toast.toast_type {
                        ToastType::Success => "Success:",
                        ToastType::Error => "Error:",
                        ToastType::Warning => "Warning:",
                        ToastType::Info => "Information:",
                    },
                    {toast.toast_type.icon_component()}
                }

                div {
                    class: "flex-1 space-y-1",

                    div {
                        class: "text-sm font-semibold leading-none tracking-tight",
                        id: format!("toast-title-{}", toast.id),
                        "{toast.title}"
                    }

                    if let Some(description) = &toast.description {
                        div {
                            class: "text-sm opacity-90",
                            id: format!("toast-desc-{}", toast.id),
                            "{description}"
                        }
                    }
                }
            }

            Button {
                variant: ButtonVariant::Ghost,
                is_icon_button: true,
                aria_label: Some("Close".to_string()),
                on_click: start_exit,
                class: "absolute right-2 top-2 opacity-0 group-hover:opacity-100",
                X { class: "size-4" }
            }
        }
    }
}

// Toast options struct for easier API
#[derive(Clone, Default)]
pub struct ToastOptions {
    pub description: Option<String>,
    pub duration: Option<Duration>,
    pub permanent: bool,
}

// Simplified toast API
#[derive(Clone, Copy)]
pub struct Toasts;

impl Toasts {
    // Show a toast with the given type and options
    pub fn show(&self, title: String, toast_type: ToastType, options: ToastOptions) {
        let mut next_id = NEXT_ID.write();
        let id = *next_id;
        *next_id += 1;

        let toast = ToastItem {
            id,
            title,
            description: options.description,
            toast_type,
            duration: if options.permanent {
                None
            } else {
                options.duration
            },
            permanent: options.permanent,
            visible: true,
        };

        let mut toasts = TOASTS.write();
        toasts.push(toast);

        // Limit the number of toasts
        while toasts.len() > 10 {
            // Try to remove non-permanent toasts first
            if let Some(pos) = toasts.iter().position(|t| !t.permanent) {
                toasts.remove(pos);
            } else {
                toasts.remove(0);
            }
        }
    }

    // Convenience methods for different toast types
    pub fn success(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Success, options.unwrap_or_default());
    }

    pub fn error(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Error, options.unwrap_or_default());
    }

    pub fn warning(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Warning, options.unwrap_or_default());
    }

    pub fn info(&self, title: String, options: Option<ToastOptions>) {
        self.show(title, ToastType::Info, options.unwrap_or_default());
    }
}

// Hook to use the toast API
pub fn use_toast() -> Toasts {
    Toasts
}
