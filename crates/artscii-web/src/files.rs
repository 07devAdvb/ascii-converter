//! Browser file IO: uploads, downloads, and clipboard.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Event, File};

use crate::state::AppState;

/// Decode an uploaded image file into app state.
///
/// Reads via the `Blob.arrayBuffer()` promise: `FileReader.result()` yields
/// an `ArrayBuffer`, not a `Uint8Array`, so the old `dyn_ref` cast failed
/// silently.
pub fn read_file(file: File, state: AppState) {
    if !file.type_().starts_with("image/") {
        state
            .error
            .set(Some("unsupported file type — please drop an image".into()));
        return;
    }

    state.error.set(None);
    state.file_name.set(Some(file.name()));

    // Thumbnail preview via object URL; revoke the previous one to avoid leaks.
    if let Some(old) = state.thumb_url.get_untracked() {
        let _ = web_sys::Url::revoke_object_url(&old);
    }
    if let Ok(url) = web_sys::Url::create_object_url_with_blob(&file) {
        state.thumb_url.set(Some(url));
    }

    spawn_local(async move {
        match JsFuture::from(file.array_buffer()).await {
            Ok(buf) => {
                let bytes = js_sys::Uint8Array::new(&buf).to_vec();
                match image::load_from_memory(&bytes) {
                    Ok(img) => state.image.set(Some(img)),
                    Err(_) => state.error.set(Some("couldn't decode that image".into())),
                }
            }
            Err(_) => state.error.set(Some("couldn't read that file".into())),
        }
    });
}

/// Handle a file `<input>` change event by loading the selected file.
pub fn on_file_input(ev: Event, state: AppState) {
    let input = event_target::<web_sys::HtmlInputElement>(&ev);
    if let Some(file) = input.files().and_then(|f| f.get(0)) {
        read_file(file, state);
    }
    // Allow re-selecting the same file.
    input.set_value("");
}

/// Trigger a browser download of `content` under `name`.
pub fn download_file(name: &str, mime: &str, content: &str) {
    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(content));

    let bag = web_sys::BlobPropertyBag::new();
    bag.set_type(mime);
    let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &bag) else {
        return;
    };
    let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else {
        return;
    };
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(a) = document.create_element("a") else {
        return;
    };

    let _ = a.set_attribute("href", &url);
    let _ = a.set_attribute("download", name);
    if let Ok(elem) = a.dyn_into::<web_sys::HtmlElement>() {
        elem.click();
    }
    let _ = web_sys::Url::revoke_object_url(&url);
}

/// Copy `text` to the clipboard and flash `copied` for 1.5 seconds.
pub fn copy_to_clipboard(text: String, copied: RwSignal<bool>) {
    spawn_local(async move {
        let Some(window) = web_sys::window() else {
            return;
        };
        let _ = JsFuture::from(window.navigator().clipboard().write_text(&text)).await;
        copied.set(true);
        let cb = wasm_bindgen::closure::Closure::once(move || copied.set(false));
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            1500,
        );
        cb.forget();
    });
}
