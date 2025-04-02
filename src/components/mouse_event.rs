use dioxus::prelude::*;
use ropey::Rope;

pub fn handle_mouse_click(
    e: MouseEvent,
    mut cursor_position: Signal<(usize, usize)>,
    line_height: usize,
    char_width: usize,
    rope: &Rope,
) {
    // 获取鼠标点击位置
    let coordinates = e.page_coordinates();
    let mouse_x = coordinates.x.round() as usize;
    let mouse_y = coordinates.y.round() as usize;

    // 将像素位置转换为行和列位置
    let mut line = (mouse_y - 41) / line_height;
    let mut col = 0;
    let mut width = 0;

    // 获取最大行号
    let max_lines = rope.len_lines();
    if line >= max_lines - 1 {
        line = max_lines - 2;
    }
    println!("Max lines: {}", max_lines);

    if line < max_lines {
        let line_text = rope.line(line);
        let max_cols = line_text.len_chars();
        for (i, ch) in line_text.chars().enumerate() {
            width += if ch.is_ascii() {
                char_width
            } else {
                char_width * 2
            };
            if width > mouse_x - 8 {
                col = i;
                break;
            }
        }
        // 如果列号超出当前行的最大位置，设置列号为行尾
        if mouse_x - 8 >= width {
            col = max_cols - 1;
        }
    }

    // 更新光标位置
    cursor_position.set((line, col));
    println!("Cursor position: {:?}", cursor_position());
}
