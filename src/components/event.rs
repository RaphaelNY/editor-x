use dioxus::prelude::*;

pub fn handle_mouse_click(
    e: MouseEvent,
    mut cursor_position: Signal<(usize, usize)>,
) {
    // 获取鼠标点击位置
    let corrdinates = e.data.page_coordinates();
	let mouse_x = corrdinates.x.round() as usize;
	let mouse_y = corrdinates.y.round() as usize;

    // 更新光标位置
    cursor_position.set((mouse_y, mouse_x));
}
