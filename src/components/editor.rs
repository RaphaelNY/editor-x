use dioxus::prelude::*;

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
impl Token {
    pub fn default(text: &'static str) -> Token {
        Self {
            text,
            color: "#005",
            background: "#fff",
            is_cursor: false,
        }
    }
    pub fn _new(text: &'static str, color: &'static str, background: &'static str) -> Token {
        Self {
            text,
            color,
            background,
            is_cursor: false,
        }
    }
}

#[component]
pub fn EditorArea(props: EditorAreaProps) -> Element {
    // selected language
    let mut _language = props.language.clone();
    let cursor_position = props.cursor_position.clone();

    let mut lines = vec![
        vec![
            Token {
                text: "fn",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: " ",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: "main",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: "(",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: ")",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: " ",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: "{",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
        ],
        vec![
            Token {
                text: "    ",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: "println",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: "!",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: r#""Hello, World!""#,
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
            Token {
                text: ";",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            },
        ],
        vec![
            Token {
                text: "}",
                color: "#000",
                background: "#fff",
                is_cursor: false,
            }
        ],
    ];

    // 更新光标位置，设置当前光标为特定位置
    let mut lines_with_cursor = lines.clone();
    let (cursor_row, cursor_col) = cursor_position();
    // 处理光标从数字位置转换为具体的token位置
    if cursor_row < lines_with_cursor.len() {
        let len = lines_with_cursor
            .iter()
            .map(|line| line.iter().map(|token| token.text.len()).sum::<usize>())
            .sum::<usize>();
        if cursor_col < len {
            let mut lenx = 0;
            for (index, token) in lines_with_cursor[cursor_row].iter().enumerate() {
                if cursor_col > lenx && cursor_col > lenx + token.text.len(){
                    lenx += token.text.len();
                } else if cursor_col > lenx && cursor_col <= lenx + token.text.len() {
                    let mut tokenLeft = Token::default(&token.text[..(cursor_col - lenx)]);
                    let tokenRight = Token::default(&token.text[(cursor_col - lenx)..]);
                    tokenLeft.is_cursor = true;
                    // let token = Token::default(contents);
                    lines_with_cursor[cursor_row].splice(index..=index, vec![tokenLeft, tokenRight]);
                    break;
                }
            }
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
        for (_i, line) in lines_with_cursor.iter().enumerate() {
            div {
                style: "white-space: pre; font-family: monospace; font-size: 16px; padding: 4px;",
                for (_j, token) in line.iter().enumerate() {
                    span {
                        style: format!(
                            "color: {}; background: {}; {}",
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
                }
            }
        }
    }
}
