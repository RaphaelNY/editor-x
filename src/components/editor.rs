use dioxus::{html::{g::end, input::max}, prelude::*};
use crate::components::handle_mouse_click;

#[derive(Props, PartialEq, Clone)]
pub struct EditorAreaProps {
    language: Signal<String>,
    cursor_position: Signal<(usize, usize)>,
}

#[derive(Props, PartialEq, Clone)]
pub struct TextareaProps {
    on_input: Callback<Event<FormData>>,
    is_composing: Signal<bool>,
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
    
    pub fn update_color(&mut self, color: &'static str) {
        self.color = color;
    }
    
    pub fn update_background(&mut self, background: &'static str) {
        self.background = background;
    }

    pub fn text_len(&self) -> usize {
        self.text.chars().count()
    }

    pub fn byte_len(&self) -> usize {
        self.text.bytes().count()
    }

    pub fn display_len(&self) -> usize {
        self.text.chars().map(|c| {
            if c.is_ascii() {
                1 // ASCII字符按1个字符计算
            } else {
                2 // 非ASCII字符（如汉字）按2个字符计算
            }
        }).sum()
    } 
}

const CHAR_WIDTH: f32 = 10.0; // 假设每个字符10px宽
const LINE_HEIGHT: f32 = 26.0; // 假设行高26px

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let on_input = props.on_input.clone();
    let mut is_composing = props.is_composing.clone();
    rsx! {
        textarea {
            // maxlength: 1,
            oninput: on_input.clone(),
            oncompositionstart: move |_| is_composing.set(true),
            oncompositionend: move |_| is_composing.set(false),
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
    let is_composing = use_signal(|| false);

    let mut input_buffer = use_signal(|| (String::new(), 0));
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
        let (cursor_px_y, cursor_px_x) = cursor_position();
        let cursor_row = (cursor_px_y - 65) / LINE_HEIGHT as usize;
        let cursor_col = (cursor_px_x) / CHAR_WIDTH as usize;
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
                .map(|line| line.iter().map(|token| token.display_len()).sum::<usize>())
                .sum::<usize>(); // byte length
            if cursor_col < len {
                for (index, token) in lines_with_cursor[cursor_row].iter().enumerate() {
                    let token_display_len = token.display_len();
                    if cursor_col >= lenx && cursor_col < lenx + token_display_len {
                        let display_offset_in_token = cursor_col - lenx;
                        let mut byte_offset_in_token = 0;

                        // 显示宽度转换成字节偏移量
                        for (i, c) in token.text.chars().enumerate() {
                            let char_width = if c.is_ascii() { 1 } else { 2 };
                            if byte_offset_in_token + char_width > display_offset_in_token {
                                byte_offset_in_token = token.text[0..i].bytes().count();
                                break;
                            }
                            byte_offset_in_token += char_width; 
                        }

                        let token_left_text = &token.text[..byte_offset_in_token];
                        let token_right_text = &token.text[byte_offset_in_token..];
        
                        let mut token_left = Token::default(token_left_text.to_string());
                        token_left.is_cursor = true;
        
                        let token_right = Token::default(token_right_text.to_string());
                        // 将分割的 token 插入
                        col.set(index);
                        lines_with_cursor[cursor_row].splice(index..=index, vec![token_left, token_right]);
                        break;
                    }
                    lenx += token_display_len;
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
        input_buffer.set((String::new(), 0));
    };

    // 更新缓存中的内容并进行输入
    let on_input = move |e: Event<FormData>| {
        if is_composing() {
            return;
        }
        let current_value = e.value();
        let (mut prev_value, _) = input_buffer();

        let new_chars: Vec<char> = if text_len(&current_value) > text_len(&prev_value) {
            let prev_char_count = prev_value.chars().count();
            let new_chars_iter = current_value.chars().skip(prev_char_count);
            new_chars_iter.collect() 
        } else {
            vec![]
        };
        
        println!("current_value: {:?}", current_value);
        println!("prev_value: {:?}", prev_value);
        println!("new_chars: {:?}", new_chars);

        prev_value = current_value.clone();
        input_buffer.set((prev_value, 0));

        let mut lines_with_cursor = lines();
        lines_with_cursor[row()][col()].update_text(String::from_iter(new_chars).clone());
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
                            if token.is_cursor && col() <= cursor_line.iter().map(|token| token.text_len()).sum::<usize>() {
                                Textarea {
                                    on_input: on_input.clone(),
                                    is_composing: is_composing.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn text_len(text: &String) -> usize {
    text.chars().count()
}