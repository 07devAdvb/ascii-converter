use leptos::prelude::*;

use crate::components::{Footer, Header, Hero, Toast, Workspace};
use crate::state::AppState;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    view! {
        <div class="app">
            <Header/>

            <Show when=move || state.image.with(|i| i.is_none())>
                <Hero/>
            </Show>
            <Show when=move || state.image.with(|i| i.is_some())>
                <Workspace/>
            </Show>

            <Toast/>
            <Footer/>
        </div>
    }
}
