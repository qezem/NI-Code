use dioxus::prelude::*;

use crate::pages::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CurrentPage {
    Home,
    Editor
}

#[component]
pub fn App() -> Element {
    let mut current_page = use_signal(|| CurrentPage::Home);

    match current_page() {
        CurrentPage::Home => rsx! {
            HomePage { current_page }
        },
        CurrentPage::Editor => rsx! {
            EditorPage {}
        }
    }
}