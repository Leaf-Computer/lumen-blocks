use dioxus_lib::prelude::*;
use dioxus_primitives::toast::{
    ToastProvider as PrimitiveToastProvider,
};

/// Re-export for convenience
pub use dioxus_primitives::toast::{use_toast, ToastOptions, ToastType};

/// Styled ToastProvider wrapper that includes the container
#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    pub children: Element,
}

#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    rsx! {
        PrimitiveToastProvider {
            {props.children}
        }
    }
}