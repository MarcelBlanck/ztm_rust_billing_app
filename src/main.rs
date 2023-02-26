// Project 1: Interactive bill manager
//
// Extracted from the exercise description:
//
// User stories:
// * Stage 1:
//   - As a user I want to add bills with name and amount owed.
//   - As a user I want to view all bills in a list.
// * Stage 2:
//   - As a user I want to delete bills.
// * Stage 3:
//   - As a user I want to edit existing bills.
//   - As a user I want to go to the main menu, if I change my mind.

use command::Command as Cmd;
use database::{Data, Database};
use screens::Screens;
pub mod command;
pub mod database;
pub mod screens;

#[derive(Clone)]
struct Bill {
    customer: String,
    amount: f64,
}

fn bill_data_to_text(data_list: Vec<Data<Bill>>) -> String {
    let mut text = String::new();
    for d in data_list {
        text.push_str(&format!(
            "{}. {}, {}\n",
            d.id, d.data.customer, d.data.amount
        ));
    }
    text
}

fn main() {
    let mut db = Database::<Bill>::default();
    let mut screens = Screens::default();
    let mut screen = screens.get_main_menu();

    loop {
        screen.show();
        match screen.process_input() {
            Cmd::ShowMainMenu | Cmd::Back => screen = screens.get_main_menu(),
            Cmd::ShowAddScreen => screen = screens.get_add_bill(),
            Cmd::ShowBillListScreen => {
                let text = bill_data_to_text(db.as_list());
                screen = screens.get_bill_list(text);
            }
            Cmd::ShowDeleteScreen => {
                let text = bill_data_to_text(db.as_list());
                screen = screens.get_delete_bill(text);
            }
            Cmd::ShowEditScreen => {
                let text = bill_data_to_text(db.as_list());
                screen = screens.get_edit_bill(text);
            }
            Cmd::NotifyError { err } => {
                println!("ERROR: {}", err);
                screen = screens.get_main_menu();
            }
            Cmd::Add { customer, amount } => {
                db.add(Bill { customer, amount });
                screen = screens.get_main_menu();
            }
            Cmd::Delete { id } => {
                db.remove(&id);
                screen = screens.get_main_menu();
            }
            Cmd::Edit {
                id,
                customer,
                amount,
            } => {
                db.update(&id, Bill { customer, amount });
                screen = screens.get_main_menu();
            }
            Cmd::Exit => break,
        }
    }
}
