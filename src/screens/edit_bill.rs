use super::screen::Screen;
use crate::command::Command;

pub struct EditBillScreen {
    text: String,
}

impl EditBillScreen {
    pub fn default() -> Self {
        EditBillScreen {
            text: String::new(),
        }
    }

    fn parse_bill(csv: &str) -> Option<(usize, String, f64)> {
        let split_text: Vec<&str> = csv.split(',').collect();
        println!("{:?}", split_text);
        if split_text.len() != 3 {
            None
        } else {
            let customer = split_text[1].trim().to_owned();
            if let Ok(id) = split_text[0].trim().parse::<usize>() {
                if let Ok(amount) = split_text[2].trim().parse::<f64>() {
                    return Some((id, customer, amount));
                }
            };
            None
        }
    }
}

impl Screen for EditBillScreen {
    fn show(&self) {
        Self::clear_screen_and_reposition_cursor();
        Self::print_header("EDIT BILL");
        print!("{}", self.text);
        println!("Id, Name, Amount?");
    }

    fn process_input(&self) -> Command {
        let input = Self::get_input();
        match input {
            Ok(None) => Command::Back,
            Ok(Some(s)) => {
                if let Some(t) = Self::parse_bill(s.as_str()) {
                    Command::Edit {
                        id: t.0,
                        customer: t.1,
                        amount: t.2,
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

    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_for_good_input() {
        let result = EditBillScreen::parse_bill("3,test,1.337").unwrap();
        assert_eq!(result.0, 3);
        assert_eq!(result.1, "test");
        assert_eq!(result.2, 1.337);

        let result = EditBillScreen::parse_bill("3,test,1337").unwrap();
        assert_eq!(result.0, 3);
        assert_eq!(result.1, "test");
        assert_eq!(result.2, 1337.0);
    }

    #[test]
    fn test_parser_for_bad_input() {
        let result = EditBillScreen::parse_bill("");
        assert!(result.is_none());
        let result = EditBillScreen::parse_bill("w,orl,d");
        assert!(result.is_none());
        let result = EditBillScreen::parse_bill("w,test,1.337");
        assert!(result.is_none());
        let result = EditBillScreen::parse_bill("1,test,d");
        assert!(result.is_none());
        let result = EditBillScreen::parse_bill("1.5,test,1.337");
        assert!(result.is_none());
    }
}
