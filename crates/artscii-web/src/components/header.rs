use leptos::prelude::*;

const LOGO: &str = r"   _  ___ _____ ___  ___ ___ ___
  /_\ | _ \_   _/ __|/ __|_ _|_ _|
 / _ \|   / | | \__ \ (__ | | | |
/_/ \_\_|_\ |_| |___/\___|___|___|";

/// Top bar with the ASCII logo and project link.
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="topbar">
            <pre class="logo" aria-hidden="true">{LOGO}</pre>
            <div class="tagline">
                <span>"image → ascii"</span>
                <span class="dim">" · in-browser wasm · "</span>
                <a
                    href="https://github.com/4ster-light/artscii"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    "[src]"
                </a>
            </div>
        </header>
    }
}
