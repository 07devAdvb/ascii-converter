use leptos::prelude::*;

use crate::files::{on_file_input, read_file};
use crate::state::AppState;

/// Landing view: drag-and-drop zone shown until an image is loaded.
#[component]
pub fn Hero() -> impl IntoView {
    let state = expect_context::<AppState>();

    view! {
        <section class="hero">
            <div
                class=move || {
                    if state.is_dragging.get() { "drop dragging" } else { "drop" }
                }
                on:dragover=move |ev| {
                    ev.prevent_default();
                    state.is_dragging.set(true);
                }
                on:dragleave=move |_| state.is_dragging.set(false)
                on:drop=move |ev| {
                    ev.prevent_default();
                    state.is_dragging.set(false);
                    if let Some(file) = ev
                        .data_transfer()
                        .and_then(|dt| dt.files())
                        .and_then(|f| f.get(0))
                    {
                        read_file(file, state);
                    }
                }
            >
                <input
                    type="file"
                    id="file-hero"
                    accept="image/*"
                    class="visually-hidden"
                    on:change=move |ev| on_file_input(ev, state)
                />
                <label r#for="file-hero" class="drop-inner">
                    <span class="plus" aria-hidden="true">"+"</span>
                    <span class="drop-title">"drop an image"</span>
                    <span class="drop-sub">
                        "or click to browse — png · jpeg · gif · webp · bmp"
                    </span>
                </label>
            </div>
            <p class="hint">"everything runs locally in your browser — nothing is uploaded"</p>
        </section>
    }
}
