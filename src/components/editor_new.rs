use dioxus::logger::tracing::span;
use ropey::Rope;
use dioxus::prelude::*;
use crate::components::handle_mouse_click;
use crate::praser::{parse, SyntaxBlocks, SyntaxType, TextNode};
use std::f32::consts::E;
use std::sync::{Arc, Mutex};
use tokio::task;

static DEBUG: bool = true;
static DEBUG0: bool = false;
static DEBUG1: bool = false;

const LINE_HEIGHT: usize = 26; // 根据实际情况调整
const CHAR_WIDTH: usize = 10; // 根据实际情况调整

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

#[allow(unused)]
#[derive(Debug)]
pub struct Editor {
    text: Arc<Mutex<Rope>>,
    cursor_position: (usize, usize), // (line, column)
}

#[allow(unused)]
impl Editor {
    pub fn new() -> Self {
        Editor {
            text: Arc::new(Mutex::new(Rope::new())),
            cursor_position: (0, 0),
        }
    }

    /// 获取当前文本
    pub fn get_text(&self) -> String {
        let rope = self.text.lock().unwrap();
        rope.to_string()
    }

    /// 插入文本
    pub fn insert_text(&mut self, text: &str) {
        let cursor_pos = self.cursor_position_to_byte_offset();
        {
            let mut rope = self.text.lock().unwrap();
            rope.insert(cursor_pos, text);
        }
        self.update_cursor_position_after_insert(text);
    }

    /// 删除文本
    pub fn delete_text(&mut self, length: usize) {
        let mut rope = self.text.lock().unwrap();
        let cursor_pos = self.cursor_position_to_byte_offset();
        rope.remove(cursor_pos..cursor_pos + length);
    }

    /// 更新光标位置
    fn update_cursor_position_after_insert(&mut self, text: &str) {
        let new_cursor_position = self.cursor_position.1 + text.chars().count();
        self.cursor_position = (self.cursor_position.0, new_cursor_position);
    }

    /// 根据光标位置获取字节偏移量
    fn cursor_position_to_byte_offset(&self) -> usize {
        let rope = self.text.lock().unwrap();
        let (line, col) = self.cursor_position;
        rope.line_to_char(line) + col
    }

    /// 设置光标位置
    pub fn set_cursor_position(&mut self, line: usize, col: usize) {
        self.cursor_position = (line, col);
    }

    /// 获取当前光标位置
    pub fn get_cursor_position(&self) -> (usize, usize) {
        self.cursor_position
    }

    /// 向右移动光标
    pub fn move_cursor_right(&mut self) {
        let mut cursor_position = self.cursor_position;
        let rope = self.text.lock().unwrap();

        // 如果光标没有到达行尾，向右移动
        if cursor_position.1 < rope.line(cursor_position.0).len_chars() {
            cursor_position.1 += 1;
        } else if cursor_position.0 < rope.lines().count() - 1 {
            // 向下移动光标到下一行的开头
            cursor_position.0 += 1;
            cursor_position.1 = 0;
        }

        self.cursor_position = cursor_position;
    }

    /// 向左移动光标
    pub fn move_cursor_left(&mut self) {
        let mut cursor_position = self.cursor_position;
        if cursor_position.1 > 0 {
            cursor_position.1 -= 1;
        } else if cursor_position.0 > 0 {
            // 向上移动光标到上一行的结尾
            cursor_position.0 -= 1;
            cursor_position.1 = self.text.lock().unwrap().line(cursor_position.0).len_chars();
        }

        self.cursor_position = cursor_position;
    }

    /// 向下移动光标
    pub fn move_cursor_down(&mut self) {
        let mut cursor_position = self.cursor_position;
        let rope = self.text.lock().unwrap();

        if cursor_position.0 < rope.lines().count() - 1 {
            cursor_position.0 += 1;
            cursor_position.1 = std::cmp::min(cursor_position.1, rope.line(cursor_position.0).len_chars());
        }

        self.cursor_position = cursor_position;
    }

    /// 向上移动光标
    pub fn move_cursor_up(&mut self) {
        let mut cursor_position = self.cursor_position;
        if cursor_position.0 > 0 {
            cursor_position.0 -= 1;
            cursor_position.1 = std::cmp::min(cursor_position.1, self.text.lock().unwrap().line(cursor_position.0).len_chars());
        }

        self.cursor_position = cursor_position;
    }

    /// 换行
    pub fn move_cursor_enter(&mut self) {
        let cursor_pos = self.cursor_position_to_byte_offset();
        {
            let mut rope = self.text.lock().unwrap();
            rope.insert(cursor_pos, "\n");
        }
        self.cursor_position = (self.cursor_position.0 + 1, 0);
    }

    /// 删除字符(delete)
    pub fn move_cursor_delete(&mut self, length: usize) {
        let mut rope = self.text.lock().unwrap();
        let cursor_pos = self.cursor_position_to_byte_offset();
        if cursor_pos + length <= rope.len_chars() {
            rope.remove(cursor_pos..cursor_pos + length);
        }
    }

    /// 删除字符(backspace)
    pub fn move_cursor_backspace(&mut self, length: usize) {
        let mut rope = self.text.lock().unwrap();
        let cursor_pos = self.cursor_position_to_byte_offset();
        if cursor_pos >= length {
            rope.remove(cursor_pos - length..cursor_pos);
        }
        self.cursor_position = (self.cursor_position.0, self.cursor_position.1.saturating_sub(length));
    }

    pub fn move_cursor_tab(&mut self) {
        self.insert_text("    ");
        self.cursor_position = (self.cursor_position.0, self.cursor_position.1);
    }

    /// 解析文本并返回语法块
    pub fn parse_text(&self) -> SyntaxBlocks {
        let rope = self.text.lock().unwrap();
        let mut syntax_blocks = SyntaxBlocks::default();
        parse(&rope, &mut syntax_blocks);
        syntax_blocks
    }

	/// 检查光标是否在给定行列
	pub fn is_cursor_at(&self, line: usize, col: usize) -> bool {
		self.cursor_position == (line, col)
	}
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TextAreaProps {
    pub on_input: Callback<Event<FormData>>,
    pub editor: Signal<Editor>,
}
#[component]
pub fn Textarea(props: TextAreaProps) -> Element {
    rsx! {
        textarea {
            style: format!(
                "position: absolute; top: {}px; left: {}px; width: 1px; height: {}px; opacity: 0;",
                props.editor.with(|e| e.cursor_position.0 * LINE_HEIGHT + 65),
                props.editor.with(|e| (e.cursor_position.1 + 1) * CHAR_WIDTH),
                LINE_HEIGHT
            ),
            oninput: props.on_input.clone(),
            autofocus: true,
            onmounted: move |evt| {
                let _ = evt.data.set_focus(true);
            },
        },
    }
}

#[component]
pub fn EditorArea(props: EditorAreaProps) -> Element {
	let mut editor = use_signal(|| {
        let mut editor = Editor::new();
        // 插入调试文本
        editor.insert_text("fn main() {\n    println!(\"Hello, world!\");\n}\n");
		editor.set_cursor_position(0, 0);
        editor
    });
    let cursor_position = props.cursor_position.clone();
    let is_handled_by_keydown = Signal::new(false);

    let on_click = move |e| {
        if DEBUG {
            println!("Click: {:?}", e);
        }	
        editor.with(|editorx| {
            handle_mouse_click(e, cursor_position.clone(), LINE_HEIGHT, CHAR_WIDTH, &editorx.text.lock().unwrap());
        });
        let (line, col) = cursor_position();
        editor.with_mut(|editorx| editorx.set_cursor_position(line, col));
    };

    let on_keydown = {
        let mut is_handled_by_keydown = is_handled_by_keydown.clone();
        move |e: Event<KeyboardData>| {
            if DEBUG {
                println!("Keydown: {:?}", e.key());
            }
            editor.with_mut(|editorx| {
                match e.key() {
                    Key::ArrowLeft=>editorx.move_cursor_left(),
                    Key::ArrowRight=>editorx.move_cursor_right(),
                    Key::ArrowUp=>editorx.move_cursor_up(),
                    Key::ArrowDown=>editorx.move_cursor_down(),
                    Key::Enter=>editorx.move_cursor_enter(),
                    Key::Backspace=>editorx.move_cursor_backspace(1),
                    Key::Delete=>editorx.move_cursor_delete(1),
                    Key::Tab=>editorx.move_cursor_tab(),
                    _=>{}
                }
            });
            is_handled_by_keydown.set(true);
        }
    };

    let on_input = {
        let mut is_handled_by_keydown = is_handled_by_keydown.clone();
        move |e: Event<FormData>| {
            if is_handled_by_keydown() {
                is_handled_by_keydown.set(false);
                return;
            }
            if DEBUG {
                println!("Input: {:?}", e.data());
            }
            editor.with_mut(|editorx| {
                if let Some(last_char) = e.value().chars().last() {
                    if last_char != '\n' || last_char != '\t' {
                        editorx.insert_text(&last_char.to_string());
                    }
                }
            });
        }
    };

    let syntax_blocks = editor.with(|e| e.parse_text());

    rsx! {
        div {
            style: "flex: 1 1 auto; overflow: hidden; font-family: monospace; font-size: 16px;",
            onclick: on_click.clone(),
            onkeydown: on_keydown.clone(), 
            Textarea {
                on_input: on_input.clone(),
                editor: editor.clone(),
            }
            for line_index in 0..syntax_blocks.len() {
                div {
                    style: "white-space: pre; font-family: monospace; font-size: 16px; padding: 4px;",
                    if editor.with(|e| e.is_cursor_at(line_index, 0)) {
                        span {
                            style: format!("width: {}px; display: inline-block; border-right: 2px solid black;", CHAR_WIDTH),
                            " "
                        }
                    } else {
                        span {
                            style: format!("width: {}px; display: inline-block;", CHAR_WIDTH),
                            " "
                        }
                    }
                    for (col_index, (syntax_type, text_node)) in syntax_blocks.get_line(line_index).iter().enumerate() {
                        match text_node {
                            TextNode::Range(range) => {
                                let rope = editor.with(|e| e.text.lock().unwrap().clone());
                                let text = rope.slice(range.clone()).to_string();
                                let mut rendered_text = String::new();
                                let char_count = syntax_blocks.char_count_up_to(line_index, col_index); 
                                for (i, ch) in text.chars().enumerate() {
                                    if editor.with(|e| e.is_cursor_at(line_index, char_count + i + 1)) {
                                        rendered_text.push_str(&format!("<span style=\"border-right: 2px solid black;\">{}</span>", ch));
                                    } else {
                                        rendered_text.push(ch);
                                    }
                                }
                                rsx! {
                                    span {
                                        style: format!("color: {};", syntax_type.color()),
                                        dangerous_inner_html: "{rendered_text}"
                                    }
                                }
                            },
                            TextNode::LineOfChars { len, char } => {
                                let text = char.to_string().repeat(*len);
                                let char_count = syntax_blocks.char_count_up_to(line_index, col_index);
                                let mut rendered_text = String::new();
                                for (i, ch) in text.chars().enumerate() {
                                    if editor.with(|e| e.is_cursor_at(line_index, char_count + i + 1)) {
                                        rendered_text.push_str(&format!("<span style=\"border-right: 2px solid black;\">{}</span>", ch));
                                    } else {
                                        rendered_text.push(ch);
                                    }
                                }
                                rsx! {
                                    span {
                                        dangerous_inner_html: "{rendered_text}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}