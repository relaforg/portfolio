mod app;
mod projects;
mod repos;
mod terminal;

use app::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
