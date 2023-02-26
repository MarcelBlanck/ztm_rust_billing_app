mod add_bill;
mod bill_list;
mod delete_bill;
mod edit_bill;
mod main_menu;
pub mod screen;

use add_bill::AddBillScreen;
use bill_list::BillListScreen;
use main_menu::MainMenu;
use screen::Screen;

use self::{delete_bill::DeleteBillScreen, edit_bill::EditBillScreen};

pub struct Screens {
    main_menu: MainMenu,
    add_bill: AddBillScreen,
    bill_list: BillListScreen,
    delete_bill: DeleteBillScreen,
    edit_bill: EditBillScreen,
}

impl Default for Screens {
    fn default() -> Self {
        Screens {
            main_menu: MainMenu::default(),
            add_bill: AddBillScreen {},
            bill_list: BillListScreen::default(),
            delete_bill: DeleteBillScreen::default(),
            edit_bill: EditBillScreen::default(),
        }
    }
}

impl Screens {
    pub fn get_main_menu(&self) -> &dyn Screen {
        &self.main_menu
    }

    pub fn get_add_bill(&self) -> &dyn Screen {
        &self.add_bill
    }

    pub fn get_bill_list(&mut self, text: String) -> &dyn Screen {
        self.bill_list.set_text(text);
        &self.bill_list
    }

    pub fn get_delete_bill(&mut self, text: String) -> &dyn Screen {
        self.delete_bill.set_text(text);
        &self.delete_bill
    }

    pub fn get_edit_bill(&mut self, text: String) -> &dyn Screen {
        self.edit_bill.set_text(text);
        &self.edit_bill
    }
}
