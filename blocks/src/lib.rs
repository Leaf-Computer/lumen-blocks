use dioxus_lib::prelude::*;

// Re-export log crate for use in components
pub use log;

pub mod components;

/// Generate a runtime-unique id.
fn use_unique_id() -> Signal<String> {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    let id = *NEXT_ID.peek();
    let id_str = format!("dxc-{id}");

    // Update the ID counter in an effect to avoid signal writes during rendering
    use_effect(move || {
        *NEXT_ID.write() += 1;
    });

    use_signal(|| id_str)
}

// Elements can only have one id so if the user provides their own, we must use it as the aria id.
fn use_id_or(
    mut gen_id: Signal<String>,
    user_id: ReadOnlySignal<Option<String>>,
) -> Memo<Option<String>> {
    // If we have a user ID, update the gen_id in an effect
    use_effect(move || {
        if let Some(id) = user_id() {
            gen_id.set(id);
        }
    });

    // Return the appropriate ID
    use_memo(move || match user_id() {
        Some(user_id) => Some(user_id),
        None => Some(gen_id.peek().clone()),
    })
}
