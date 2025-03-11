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
            oncompositionend: move |_| {
                is_composing.set(false);
            },
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

        println!("cursor_row: {:?}, cursor_col: {:?}", cursor_row, cursor_col);
        println!("row: {:?}, col: {:?}", row(), col());

        // 处理光标从数字位置转换为具体的token位置
        if cursor_row < lines_with_cursor.len() {
            row.set(cursor_row);
            let len = lines_with_cursor[cursor_row]
                .iter()
                .map(|token| token.display_len())
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
        
                        let token_left = Token::default(token_left_text.to_string());
        
                        let token_right = Token::default(token_right_text.to_string());
                        let mut empty_token = Token::default("".to_string());
                        empty_token.is_cursor = true;
                        // 将分割的 token 插入
                        col.set(index + 1);
                        lines_with_cursor[cursor_row].splice(index..=index, vec![token_left, empty_token, token_right]);
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

        if is_composing() {
            return;
        }

        prev_value = current_value.clone();
        input_buffer.set((prev_value, 0));
        
        let mut lines_with_cursor = lines();
        if current_value.ends_with("\n") {
            let current_row = row();
            let current_col = col();
            let current_line = &mut lines_with_cursor[current_row];
            let tokens_to_move = current_line.split_off(current_col);
            lines_with_cursor.insert(current_row + 1, tokens_to_move);
            
            if lines_with_cursor[current_row].is_empty() {
                lines_with_cursor[current_row].push(Token::default("\n".to_string()));
            } 
            if lines_with_cursor[current_row + 1].is_empty() {
                lines_with_cursor[current_row + 1].push(Token::default("".to_string()));
            }

            row.set(current_row + 1);
            col.set(0);
            lines.set(lines_with_cursor);
            return;
        }

        lines_with_cursor[row()][col()].update_text(String::from_iter(new_chars).clone());
        lines.set(lines_with_cursor);
    };
    
    visualize_lines(&lines(), row(), col());

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
                        if token.is_cursor {
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

fn text_len(text: &String) -> usize {
    text.chars().count()
}

// 用于可视化 lines 中每个 token 的位置以及 is_cursor 的状态
fn visualize_lines(lines: &Vec<Vec<Token>>, row: usize, col: usize) {
    for (i, line) in lines.iter().enumerate() {
        println!("Line {}:", i);  // 输出行号
        for (j, token) in line.iter().enumerate() {
            let cursor_status = if token.is_cursor {
                " <- is_cursor"  // 如果该 token 是光标，标记
            } else {
                ""
            };

            // 输出 token 的位置、文本内容以及是否为光标
            println!(
                "    Token {} at position {}: '{}',",
                j,  // token 在当前行中的位置
                token.text, // token 的文本内容
                cursor_status  // 是否是光标位置
            );
        }
    }

    // 输出光标的当前行列位置
    println!("Cursor is at line {}, column {}.", row, col);
}
