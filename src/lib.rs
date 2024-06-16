#![feature(impl_trait_in_fn_trait_return)]

mod app;
mod constants;
mod components;
mod domain;
mod error_template;
mod pages;
mod services;
mod view;

pub mod prelude {
    pub use crate::app::App;
} 


#[cfg(test)]
mod test;

#[cfg(feature = "ssr")]
pub mod ssr {
    pub(crate) mod auth;
    pub mod database;
    pub mod fileserv;
    pub(crate) mod opponent;
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
