use super::screen::Screen;
use crate::command::Command;

pub struct DeleteBillScreen {
    text: String,
}

impl DeleteBillScreen {
    pub fn default() -> Self {
        DeleteBillScreen {
            text: String::new(),
        }
    }
}

impl Screen for DeleteBillScreen {
    fn show(&self) {
        Self::clear_screen_and_reposition_cursor();
        Self::print_header("DELETE BILL");
        print!("{}", self.text);
        println!();
    }

    fn process_input(&self) -> Command {
        let input = Self::get_input();
        match input {
            Ok(None) => Command::Back,
            Ok(Some(i)) => {
                if let Ok(id) = i.parse::<usize>() {
                    Command::Delete { id }
                } else {
                    Command::NotifyError {
                        err: "Index could not be".to_owned(),
                    }
                }
            }
            Err(err) => Command::NotifyError { err },
        }
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
