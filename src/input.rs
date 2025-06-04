
use crossterm::event::KeyCode;

pub struct InputHandler {
    pub left: bool,
    pub right: bool,
    pub shoot: bool,
    pub exit: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            shoot: false,
            exit: false,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Left => self.left = true,
            KeyCode::Right => self.right = true,
            KeyCode::Char(' ') => self.shoot = true,
            KeyCode::Esc => self.exit = true,
            _ => {}
        }
        self.exit
    }

    pub fn reset(&mut self) {
        self.left = false;
        self.right = false;
        self.shoot = false;
    }
}
