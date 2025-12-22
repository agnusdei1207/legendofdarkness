pub mod client;
pub mod server;
pub mod shared;

use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    // ... logic to show login or game ...
    view! {
        // Router will handle /login vs /game
    }
}
