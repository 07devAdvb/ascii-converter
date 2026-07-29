use leptos::prelude::*;

/// Bottom bar with the version string.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="foot">"artscii v2 · rust × wasm · mit"</footer>
    }
}
