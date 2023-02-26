use super::screen::Screen;
use crate::command::Command;

pub struct MainMenu {
    menu_items: Vec<(&'static str, Command)>,
}

impl MainMenu {
    pub fn default() -> Self {
        MainMenu {
            menu_items: vec![
                ("Add Bill", Command::ShowAddScreen),
                ("View Bills", Command::ShowBillListScreen),
                ("Remove Bill", Command::ShowDeleteScreen),
                ("Edit Bill", Command::ShowEditScreen),
                ("Exit", Command::Exit),
            ],
        }
    }
}

impl Screen for MainMenu {
    fn show(&self) {
        Self::clear_screen_and_reposition_cursor();
        Self::print_header("MAIN MENU");
        for (i, item) in self.menu_items.iter().enumerate() {
            println!("{}. {}", i + 1, item.0);
        }
        println!("---------------");
        println!("[1,2,3,4,5]?");
    }

    fn process_input(&self) -> Command {
        let input = Self::get_input();
        match input {
            Ok(None) => Command::Back,
            Ok(Some(i)) => match i.as_str() {
                "1" => self.menu_items.get(0).unwrap().1.clone(),
                "2" => self.menu_items.get(1).unwrap().1.clone(),
                "3" => self.menu_items.get(2).unwrap().1.clone(),
                "4" => self.menu_items.get(3).unwrap().1.clone(),
                "5" => self.menu_items.get(4).unwrap().1.clone(),
                _ => Command::NotifyError {
                    err: "Command not recognized".to_owned(),
                },
            },
            Err(err) => Command::NotifyError { err },
        }
    }

    fn set_text(&mut self, _: String) {}
}
