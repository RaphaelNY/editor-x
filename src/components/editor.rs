use dioxus::{html::g::cursor, prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct EditorAreaProps {
    language: Signal<String>,
    cursor_position: Signal<(usize, usize)>,
}

#[derive(Clone)]
pub struct Token {
    text: &'static str,
    color: &'static str,
    background: &'static str,
    is_cursor: bool,
}

#[component]
pub fn EditorArea(props: EditorAreaProps) -> Element {
    // selected language
    let mut language = props.language.clone();
    let cursor_position = props.cursor_position.clone();

    let lines = vec![
        vec![
            Token { text: "fn", color: "#000", background: "#fff", is_cursor: false },
            Token { text: " ", color: "#000", background: "#fff", is_cursor: false },
            Token { text: "main", color: "#000", background: "#fff", is_cursor: false },
            Token { text: "(", color: "#000", background: "#fff", is_cursor: false },
            Token { text: ")", color: "#000", background: "#fff", is_cursor: false },
            Token { text: " ", color: "#000", background: "#fff", is_cursor: false },
            Token { text: "{", color: "#000", background: "#fff", is_cursor: false },
        ],
        vec![
            Token { text: "    ", color: "#000", background: "#fff", is_cursor: false },
            Token { text: "println", color: "#000", background: "#fff", is_cursor: false },
            Token { text: "!", color: "#000", background: "#fff", is_cursor: false },
            Token { text: r#""Hello, World!""#, color: "#000", background: "#fff", is_cursor: false },
            Token { text: ";", color: "#000", background: "#fff", is_cursor: false },
        ],
        vec![
            Token { text: "}", color: "#000", background: "#fff", is_cursor: false },
        ],
    ];

    // 更新光标位置，设置当前光标为特定位置
    let mut lines_with_cursor = lines.clone();
    let (cursor_row, cursor_col) = cursor_position();
    if cursor_row < lines_with_cursor.len() {
        if cursor_col < lines_with_cursor[cursor_row].len() {
            lines_with_cursor[cursor_row][cursor_col].is_cursor = true;
        } else {
            let cursor_col = lines_with_cursor[cursor_row].len() - 1;
            lines_with_cursor[cursor_row][cursor_col].is_cursor = true;
        }
    } else {
        let cursor_row = lines_with_cursor.len() - 1;
        let cursor_col = lines_with_cursor[cursor_row].len() - 1;
        lines_with_cursor[cursor_row][cursor_col].is_cursor = true;
    }

    rsx! {
        for (i, line) in lines_with_cursor.iter().enumerate() {
            div {
                style: "white-space: pre; font-family: monospace; font-size: 16px; padding: 4px;",
                for (j, token) in line.iter().enumerate() {
                    span {
                        style: format!(
                            "color: {}; background: {}; {}",
                            token.color,
                            token.background,
                            if token.is_cursor {
                                "border-left: 2px solid #000;" // 给光标加上左边框
                            } else {
                                ""
                            }
                        ),
                        "{token.text}"
                    }
                }
            }
        }
    }
}