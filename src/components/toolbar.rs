use dioxus::prelude::*;

#[component]
pub fn Toolbar() -> Element {
    let langurages = vec!["Rust".to_string(), "Markdown".to_string()];
    let mut selected_langurage = use_signal(|| langurages[0].clone());
    rsx! {
        div {
            style: "background: #f0f0f0; padding: 8px; border-bottom: 1px solid #ddd; width: 100vw; height: 40px; display: flex; align-items: center;",

            button { class: "toolbar-btn", style: "margin-right: 4px", "新建" }
            button { class: "toolbar-btn", style: "margin-right: 4px", "打开" }
            button { class: "toolbar-btn", style: "margin-right: 4px", "保存" }
            button { class: "toolbar-btn", style: "margin-right: 4px", "另存为" }

            select { 
                style: "margin-left: auto; margin-left: 50px;",
                onchange: move |e| {
                    let target = e.value();
                    selected_langurage.set(target);
                },
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
