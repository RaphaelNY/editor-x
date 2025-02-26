use dioxus::prelude::*;
#[derive(Props, PartialEq, Clone)]
pub struct EditorAreaProps {
    language: Signal<String>,
}

#[component]
pub fn EditorArea(props: EditorAreaProps) -> Element {
    let mut content = use_signal(String::new);
    let mut lines: Signal<Vec<String>> = use_signal(Vec::new);
    let language = props.language.clone();

    // 监听 `content` 的每行更新
    use_effect(move || {
        let content_text = content();
        let new_lines = content_text
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        lines.set(new_lines);
    }); // 只有 content 改变时更新行号

    let line_number_elements = lines()
        .iter()
        .enumerate()
        .map(|(i, _)| {
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

            // 使用 textarea 进行文本输入
            textarea {
                style: "position: absolute; left: 50px; top: 0; width: calc(50vw - 50px); height: 100%; padding: 10px; font-size: 16px; line-height: 1.5; font-family: monospace; border: none; outline: none;",
                value: "{content()}", // 绑定 content 信号
                oninput: move |e| {
                    content.set(e.value().clone()); // 更新 content
                }
            }
        }
    }
}
