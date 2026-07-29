use artscii_img::AsciiResult;
use leptos::prelude::*;

use super::controls::{Check, DitheringSelect, Slider};
use crate::files::{copy_to_clipboard, download_file, on_file_input};
use crate::state::AppState;

/// Settings sidebar: file summary, conversion controls, and export actions.
#[component]
pub fn Sidebar() -> impl IntoView {
    let state = expect_context::<AppState>();
    let settings = state.settings;

    view! {
        <aside class="side">
            <FileBar/>
            <div class="group">
                <span class="group-title">"// adjust"</span>
                <Slider
                    id="res"
                    label="resolution"
                    min="0.05"
                    max="1.0"
                    step="0.01"
                    precision=2
                    value=settings.resolution
                />
                <Slider
                    id="con"
                    label="contrast"
                    min="0.1"
                    max="3.0"
                    step="0.1"
                    precision=1
                    value=settings.contrast
                />
                <Slider
                    id="bri"
                    label="brightness"
                    min="0.1"
                    max="3.0"
                    step="0.1"
                    precision=1
                    value=settings.brightness
                />
                <DitheringSelect dithering=settings.dithering/>
                <div class="checks">
                    <Check label="invert" checked=settings.inverted/>
                    <Check label="color" checked=settings.colored/>
                </div>
            </div>
            <ActionsBar/>
        </aside>
    }
}

/// Thumbnail, file name, and re-upload input.
#[component]
fn FileBar() -> impl IntoView {
    let state = expect_context::<AppState>();

    view! {
        <div class="filebar">
            <img
                class="thumb"
                src=move || state.thumb_url.get().unwrap_or_default()
                alt="uploaded source image"
            />
            <div class="filemeta">
                <span class="filename">{move || state.file_name.get().unwrap_or_default()}</span>
                <label r#for="file-side" class="change">"[ change ]"</label>
                <input
                    type="file"
                    id="file-side"
                    accept="image/*"
                    class="visually-hidden"
                    on:change=move |ev| on_file_input(ev, state)
                />
            </div>
        </div>
    }
}

/// Output dimensions and copy/save buttons.
#[component]
fn ActionsBar() -> impl IntoView {
    let state = expect_context::<AppState>();

    let copy = move |_| {
        if let Some(text) = state.plain_text.get_untracked() {
            copy_to_clipboard(text, state.copied);
        }
    };
    let save_txt = move |_| {
        if let Some(text) = state.plain_text.get_untracked() {
            download_file("ascii-art.txt", "text/plain", &text);
        }
    };
    let save_html = move |_| {
        if let Some(html) = state.result.with(|r| r.as_ref().map(AsciiResult::to_html)) {
            download_file("ascii-art.html", "text/html", &html);
        }
    };

    view! {
        <div class="side-foot">
            <span class="dims">
                {move || {
                    state
                        .dims
                        .get()
                        .map(|(w, h)| format!("{w}×{h} chars"))
                        .unwrap_or_default()
                }}
            </span>
            <div class="actions">
                <button class="btn" on:click=copy>
                    {move || if state.copied.get() { "copied" } else { "copy" }}
                </button>
                <button class="btn" on:click=save_txt>"save .txt"</button>
                <button class="btn" on:click=save_html>"save .html"</button>
            </div>
        </div>
    }
}
