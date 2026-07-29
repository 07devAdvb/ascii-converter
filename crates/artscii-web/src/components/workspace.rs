use leptos::prelude::*;

use super::preview::Preview;
use super::sidebar::Sidebar;

/// Main view once an image is loaded: settings sidebar plus live preview.
#[component]
pub fn Workspace() -> impl IntoView {
    view! {
        <main class="workspace">
            <Sidebar/>
            <Preview/>
        </main>
    }
}
