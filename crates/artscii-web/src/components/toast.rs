use leptos::prelude::*;

use crate::state::AppState;

/// Floating error banner shown whenever an error is set.
#[component]
pub fn Toast() -> impl IntoView {
    let state = expect_context::<AppState>();

    view! {
        <Show when=move || state.error.with(|e| e.is_some())>
            <div class="toast" role="alert">
                {move || state.error.get().unwrap_or_default()}
            </div>
        </Show>
    }
}
