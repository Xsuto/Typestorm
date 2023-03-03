use ncurses::{stdscr, wmove};

#[derive(Default)]
pub struct CursorPosition {
    x: usize,
    line_position: usize,
    previous_line_x: Vec<usize>,
}

impl CursorPosition {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn get_line_position(&self) -> usize {
        self.line_position
    }
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn move_left(&mut self) {
        assert_ne!(self.x,0);
        self.x -= 1;
    }
    pub fn move_right(&mut self) {
        self.x += 1;
    }
    pub fn move_to_new_line(&mut self) {
        self.previous_line_x.push(self.x - 1);
        self.line_position += 1;
        self.x = 0;
    }
    pub fn go_back_to_old_line(&mut self) {
        assert_ne!(self.line_position, 0);
        self.line_position -= 1;
        self.x = self.previous_line_x[self.line_position];
    }
    pub fn display(&self) {
        wmove(stdscr(), 0, self.x as i32);
    }
}
