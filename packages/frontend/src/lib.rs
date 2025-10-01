mod app;

pub use app::App;
use dioxus::prelude::*;

pub fn launch_frontend(app: fn() -> Element) {
    dioxus::launch(app);
}
