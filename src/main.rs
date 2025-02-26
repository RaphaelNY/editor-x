use components::{EditorArea, Toolbar};
use dioxus::prelude::*;

mod components;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let language = use_signal(|| "Rust".to_string());
    rsx! {
        div {
            style: "width: 100vw; height: 100vh; display: flex; flex-direction: column;",

            Toolbar {language: language },

            EditorArea {language: language }
        }
    }
}
