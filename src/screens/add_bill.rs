use super::screen::Screen;
use crate::command::Command;

pub struct AddBillScreen {}

impl AddBillScreen {
    fn parse_bill(csv: &str) -> Option<(String, f64)> {
        let split_text: Vec<&str> = csv.split(',').collect();
        println!("{:?}", split_text);
        if split_text.len() != 2 {
            None
        } else if let Ok(amount) = split_text[1].trim().parse() {
            Some((split_text[0].trim().to_owned(), amount))
        } else {
            None
        }
    }
}

impl Screen for AddBillScreen {
    fn show(&self) {
        Self::clear_screen_and_reposition_cursor();
        Self::print_header("ADD BILL");
        println!("Name, Amount?");
    }

    fn process_input(&self) -> Command {
        let input = Self::get_input();
        match input {
            Ok(None) => Command::Back,
            Ok(Some(s)) => {
                if let Some(t) = Self::parse_bill(s.as_str()) {
                    Command::Add {
                        customer: t.0,
                        amount: t.1,
                    }
                } else {
                    Command::NotifyError {
                        err: "Parse error. Valid example: \"Marcel Blanck, 1.55\"".to_owned(),
                    }
                }
            }
            Err(err) => Command::NotifyError { err },
        }
    }

    fn set_text(&mut self, _: String) {}
}
