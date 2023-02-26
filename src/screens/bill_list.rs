use super::screen::Screen;
use crate::command::Command;

pub struct BillListScreen {
    text: String,
}

impl BillListScreen {
    pub fn default() -> Self {
        BillListScreen {
            text: String::new(),
        }
    }
}

impl Screen for BillListScreen {
    fn show(&self) {
        Self::clear_screen_and_reposition_cursor();
        Self::print_header("BILL LIST");
        print!("{}", self.text);
    }

    fn process_input(&self) -> Command {
        Self::wait_for_any_key();
        Command::ShowMainMenu
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
