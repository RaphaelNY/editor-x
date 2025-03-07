use std::rc::Rc;

use dioxus::prelude::*;
use crate::components::handle_mouse_click;

#[derive(Props, PartialEq, Clone)]
pub struct EditorAreaProps {
    language: Signal<String>,
    cursor_position: Signal<(usize, usize)>,
}

#[derive(Props, PartialEq, Clone)]
pub struct TextareaProps {
    on_input: Callback<Event<FormData>>,
}

#[derive(Clone)]
pub struct Token {
    pub text: String,
    color: &'static str,
    background: &'static str,
    is_cursor: bool,
    is_input: bool,
}
impl Token {
    pub fn default(text: String) -> Token {
        Self {
            text,
            color: "#020",
            background: "#fff",
            is_cursor: false,
            is_input: false,
        }
    }
    pub fn _new(text: String, color: &'static str, background: &'static str) -> Token {
        Self {
            text,
            color,
            background,
            is_cursor: false,
            is_input: false,
        }
    }
    pub fn  update_text(&mut self, text: String) {
        self.text = self.text.clone() + &text;
    }
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let on_input = props.on_input.clone();
    rsx! {
        textarea {
            oninput: on_input.clone(),
            onmounted: move |evt| {
                // 挂载时自动存储引用
                let _ = evt.data.set_focus(true);
            },
            autofocus: true,
            style: "position: absolute; opacity: 0; width: 1px; height: 18px; border: none; outline: none; padding: 0;",
        }
    }
}

#[component]
pub fn EditorArea(props: EditorAreaProps) -> Element {
    // selected language
    let mut _language = props.language.clone();
    let cursor_position = props.cursor_position.clone();

    let input_buffer = use_signal(String::new);
    let mut row= use_signal(|| 0);
    let mut col= use_signal(|| 0);
    let mut lines = use_signal(|| vec![
        vec![
            Token::default("fn".to_string()),
            Token::default(" ".to_string()),
            Token::default("main".to_string()),
            Token::default("(".to_string()),
            Token::default(")".to_string()),
            Token::default(" ".to_string()),
            Token::default("{".to_string()),
        ],
        vec![
            Token::default("    ".to_string()),
            Token::default("println".to_string()),
            Token::default("!".to_string()),
            Token::default(r#""Hello, World!""#.to_string()),
            Token::default(";".to_string()),
        ],
        vec![
            Token::default("}".to_string()),
        ],
    ]);

    let on_click = move |e| {
        handle_mouse_click(e, cursor_position);
        println!("{:?}", cursor_position());

        // 更新光标位置，设置当前光标为特定位置
        let (cursor_rows, cursor_cols) = cursor_position();
        let cursor_row = (cursor_rows - 65) / 26 as usize;
        let cursor_col = (cursor_cols) / 10 as usize;
        let mut lenx = 0;

        let mut lines_with_cursor = lines();

        for line in lines_with_cursor.iter_mut() {
            for token in line.iter_mut() {
                token.is_cursor = false; // Reset cursor flag to false
            }
        }
        // 处理光标从数字位置转换为具体的token位置
        if cursor_row < lines_with_cursor.len() {
            row.set(cursor_row);
            let len = lines_with_cursor
                .iter()
                .map(|line| line.iter().map(|token| token.text.len()).sum::<usize>())
                .sum::<usize>();
            if cursor_col < len {
                for (index, token) in lines_with_cursor[cursor_row].iter().enumerate() {
                    if cursor_col > lenx && cursor_col > lenx + token.text.len(){
                        lenx += token.text.len();
                    } else if cursor_col > lenx && cursor_col <= lenx + token.text.len() {
                        let mut tokenLeft = Token::default(token.text[..(cursor_col - lenx)].to_string());
                        let tokenRight = Token::default(token.text[(cursor_col - lenx)..].to_string());
                        tokenLeft.is_cursor = true;
                        // let token = Token::default(contents);
                        col.set(index);
                        lines_with_cursor[cursor_row].splice(index..=index, vec![tokenLeft, tokenRight]);
                        break;
                    }
                }
            } else {
                let cursor_col = lines_with_cursor[cursor_row].len() - 1;
                col.set(cursor_col);
                lines_with_cursor[cursor_row][cursor_col].is_cursor = true;
            }
        } else {
            let cursor_row = lines_with_cursor.len() - 1;
            row.set(cursor_row);
            let cursor_col = lines_with_cursor[cursor_row].len() - 1;
            col.set(cursor_col);
            lines_with_cursor[cursor_row][cursor_col].is_cursor = true;
        }
        lines.set(lines_with_cursor);
    };

    // 更新缓存中的内容并进行输入
    let on_input = move |e: Event<FormData>| {
        let new_char = e.value();

        let mut lines_with_cursor = lines();
        lines_with_cursor[row()][col()].update_text(new_char.clone());
        lines.set(lines_with_cursor);
    };
    
    rsx! {
        div {
            onclick: on_click.clone(),
            for (_i, line) in lines.iter().enumerate() {
                div {
                    style: "white-space: pre; font-family: monospace; font-size: 16px; padding: 4px;",
                    for (_j, token) in line.iter().enumerate() {
                        span {
                            style: format!(
                                "color: {}; background: {}; {} font-family: monospace;",
                                token.color,
                                token.background,
                                if token.is_cursor {
                                    "border-right: 2px solid #000;" // 给光标加上右边框
                                } else {
                                    ""
                                }
                            ),
                            "{token.text}"
                        }
                        // Render a textarea when cursor is active
                        if let Some(cursor_line) = lines.get(row()) {
                            if token.is_cursor && col() <= cursor_line.iter().map(|token| token.text.len()).sum::<usize>() {
                                Textarea {
                                    on_input: on_input.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}