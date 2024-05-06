pub mod app;
pub mod components;
pub mod domain;
pub mod error_template;
pub mod pages;
pub mod services;
pub mod view;

#[cfg(feature = "ssr")]
pub mod ssr {
    pub mod auth;
    pub mod database;
    pub mod fileserv;

}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
