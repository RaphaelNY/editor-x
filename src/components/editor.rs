use dioxus::prelude::*;

#[component]
pub fn EditorArea() -> Element {
    let mut content = use_signal(String::new);
    let mut lines: Signal<Vec<String>> = use_signal(Vec::new);

    use_effect(move || {
        let content = content();
        let new_lines = content
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        lines.set(new_lines);
    });

    let line_number_elements = lines()
        .iter()
        .enumerate()
        .map(|(i, line)| {
            rsx! {
                div {
                    style: "text-align: center;",
                    "{i + 1}"
                }
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        div {
            style: "flex-grow: 1; position: relative; width: 100vw; height: 100vh;", // 设置宽度为窗口宽度

            div {
                style: "position: absolute; left: 0; top: 0; width: 40px; background: #f0f0f0; padding: 10px; line-height: 1.9; font-family: monospace; overflow-y: auto;",
                { line_number_elements.into_iter() }
            }

            textarea { 
                style: "position: absolute; left: 50px; top: 0; width: calc(100vw - 50px); height: 100%; padding: 10px; font-size: 16px; line-height: 1.5; font-family: monospace; border: none; outline: none;",
                value: content,
                oninput: move |e| {
                    content.set(e.value().clone());
                }
            }
        }
    }
}
