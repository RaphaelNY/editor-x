use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ToolbarProps {
    language: Signal<String>,
}

#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    let mut language = props.language.clone(); // 从父组件获取语言状态

    // 监听语言选择变化
    let on_language_change = move |e: Event<FormData>| {
        let selected = e.value();
        language.set(selected); // 更新语言选择
    };

    rsx! {
        div {
            style: "background: #f0f0f0; padding: 8px; border-bottom: 1px solid #ddd; width: 100vw; height: 40px; display: flex; align-items: center;",

            button { class: "toolbar-btn", style: "margin-right: 4px", "新建" }
            button { class: "toolbar-btn", style: "margin-right: 4px", "打开" }
            button { class: "toolbar-btn", style: "margin-right: 4px", "保存" }
            button { class: "toolbar-btn", style: "margin-right: 4px", "另存为" }

            select {
                style: "margin-left: auto; margin-left: 50px;",
                value: "{language}",
                onchange: on_language_change,
                option { value: "Rust", "Rust" }
                option { value: "Markdown", "Markdown" }
            }

            input {
                r#type: "file",
                id: "file-input",
                style: "display: none;",
                onchange: |event| {
                    // 处理文件选择...
                }
            }
        }
    }
}
