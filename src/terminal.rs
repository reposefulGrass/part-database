
// A collection of utilities to manipulate the terminal

pub enum ClearCode {
    _CursorToEnd = 0,
    _CursorToBeginning= 1,
    Everything = 2
}

pub fn clear (code: ClearCode) {
    println!("\x1b[{}J", code as u32);
}

pub fn cursor_move_to_pos (row: u32, col: u32) {
    println!("\x1b[{};{}H", row, col);
}

