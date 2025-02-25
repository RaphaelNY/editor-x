use dioxus::prelude::*;

#[component]
pub fn EditorArea() -> Element {
    let mut content = use_signal(String::new);

    rsx! {
        div {
            style: "flex-grow: 1; position: relative;",

            div {
                style: "position: absolute; width: 100%; height: 100%;
                        padding: 20px; overflow: auto; white-space: pre-wrap;",
                contenteditable: "true",
                oninput: move |e| {
                    e.prevent_default();
                    content.set(e.value().clone());
                },
                "{content}"
            }
        }
    }
}
