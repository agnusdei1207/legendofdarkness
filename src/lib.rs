pub mod app;
pub mod game;
pub mod components;
pub mod models;
pub mod domain;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(app::App);
}
