use components::{EditorArea, Toolbar};
use dioxus::prelude::*;

mod components;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "height: 100vh; display: flex; flex-direction: column;",

            Toolbar {},

            EditorArea {}
        }
    }
}
