mod praser;

use components::{EditorArea, Toolbar};
use dioxus::prelude::*;

mod components;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let language = use_signal(|| "Rust".to_string());
    let cursor_position = use_signal(|| (65, 8));
    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column; overflow: hidden;",

            Toolbar {language: language },

            EditorArea {language: language, cursor_position: cursor_position }
        }
    }
}
