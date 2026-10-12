use dioxus::prelude::*;

use crate::app::CurrentPage;

static CSS: &str = include_str!("../style/main.css");

#[component]
pub fn EditorPage() -> Element {
    rsx! {
        style { "{CSS}" }
        main {
            h1 { "Editor soon..." }
        }
    }
}