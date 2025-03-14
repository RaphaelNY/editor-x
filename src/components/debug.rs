use super::editor::Token;


// 用于可视化 lines 中每个 token 的位置以及 is_cursor 的状态
pub fn visualize_lines(lines: &Vec<Vec<Token>>, row: usize, col: usize) {
    for (i, line) in lines.iter().enumerate() {
        println!("Line {}:", i);  // 输出行号
        for (j, token) in line.iter().enumerate() {
            let cursor_status = if token.cursor_check() {
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
