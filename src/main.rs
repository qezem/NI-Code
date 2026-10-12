use dioxus::{desktop::Config, prelude::*};

mod app;
mod pages;

use app::*;

fn main() {
    dioxus::LaunchBuilder::new()
    .with_cfg(
        Config::default().with_menu(None).with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("NI Code")
                .with_decorations(false)
        )
    )
    .launch(App);
}