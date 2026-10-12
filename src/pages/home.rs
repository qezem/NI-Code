use dioxus::prelude::*;
use rand::seq::{IndexedRandom, SliceRandom};

use crate::app::CurrentPage;

const QUOTES: &[(&str, &str)] = &[
    ("No Idea Code", "It compiled. Don't touch it."),
    ("No Idea Code", "Debugging degraded."),
    ("Nightly Insomnia Code", "Sleep deprecated."),
    ("Nightly Insomnia Code", "Committed at 03:47."),
    ("Null Intelligence Code", "Ctrl+C refined. Ctrl+V perfected."),
    ("Not Implemented Code", "Ready in Sprint 99."),
    ("Naturally Inherited Code", "Suffering inherited."),
    ("Not Indexed Code", "Fans at 100%."),
    ("Negative Impact Code", "Friday deployment approved."),
    ("NI Code", "Compiling collapsed."),
];

struct Quotes<'a> {
    name: &'a str,
    quote: &'a str
}

static CSS: &str = include_str!("../style/main.css");

fn random_qoute() -> (&'static str, &'static str) {
    let mut rng = rand::rng();
    *QUOTES.choose(&mut rng).unwrap_or(&QUOTES[0])
}

#[component]
pub fn HomePage(mut current_page: Signal<CurrentPage>) -> Element {
    let (name, quote) = use_hook(random_qoute);

    rsx! {
        style { "{CSS}" }
        main {
            div { id: "main_menu_div", class: "main_menu_div",
                div { class: "name_and_quote_div",
                    h2 { "{name}" }
                    h3 { "{quote}" }
                }

                div { class: "main_menu_actions_div",
                    button {
                        onclick: move |_evt| {
                            current_page.set(CurrentPage::Editor);
                        },
                        "New file"
                    }
                    button { "Open file" }
                    button { "Open folder" }
                }
            }
        }
    }
}