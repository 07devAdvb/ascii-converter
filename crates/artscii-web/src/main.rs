mod app;
mod components;
mod files;
mod render;
mod state;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
