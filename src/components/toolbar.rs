use dioxus::prelude::*;

#[component]
pub fn Toolbar() -> Element {
    rsx! {
        div {
            style: "background: #f0f0f0; padding: 8px; border-bottom: 1px solid #ddd;",

            button { class: "toolbar-btn", "新建" }
            button { class: "toolbar-btn", "打开" }
            button { class: "toolbar-btn", "保存" }
            button { class: "toolbar-btn", "另存为" }

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
