use leptos::prelude::*;

use crate::state::{AppState, clamp_font_size};

/// Live ASCII preview with font-size zoom controls.
#[component]
pub fn Preview() -> impl IntoView {
    let state = expect_context::<AppState>();

    let zoom_out = move |_| {
        state
            .font_size
            .update(|v| *v = clamp_font_size(v.saturating_sub(1)));
    };
    let zoom_in = move |_| {
        state.font_size.update(|v| *v = clamp_font_size(*v + 1));
    };

    view! {
        <section class="preview">
            <div class="preview-bar">
                <span class="dim">"preview"</span>
                <span class="zoom">
                    <button on:click=zoom_out aria-label="decrease font size">"−"</button>
                    <span class="zoom-val">{move || format!("{}px", state.font_size.get())}</span>
                    <button on:click=zoom_in aria-label="increase font size">"+"</button>
                </span>
            </div>
            <div class="output-wrap">
                <pre
                    class="output"
                    style:font-size=move || format!("{}px", state.font_size.get())
                    inner_html=move || state.html_output.get().unwrap_or_default()
                ></pre>
            </div>
        </section>
    }
}
