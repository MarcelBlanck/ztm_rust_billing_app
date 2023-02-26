use crate::command::Command;
use std::io;
use std::io::{Read, Write};

pub trait Screen {
    fn show(&self);

    fn set_text(&mut self, text: String);

    fn process_input(&self) -> Command;

    fn print_header(title: &str)
    where
        Self: Sized,
    {
        println!();
        println!("{}", title.to_uppercase());
        println!("---------------");
    }

    fn get_input() -> Result<Option<String>, String>
    where
        Self: Sized,
    {
        let mut buffer = String::new();
        let result = io::stdin().read_line(&mut buffer);
        match result {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(buffer.trim().to_owned())),
            Err(e) => Err(e.to_string()),
        }
    }

    fn wait_for_any_key()
    where
        Self: Sized,
    {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        println!();
        write!(stdout, "Press any key to continue...").unwrap();
        stdout.flush().unwrap();
        let _ = stdin.read(&mut [0u8]).unwrap();
    }

    fn clear_screen_and_reposition_cursor()
    where
        Self: Sized,
    {
        print!("\x1B[2J\x1B[1;1H");
    }
}
