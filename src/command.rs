#[derive(Debug, Clone)]
pub enum Command {
    ShowMainMenu,
    ShowAddScreen,
    ShowBillListScreen,
    ShowDeleteScreen,
    ShowEditScreen,
    NotifyError {
        err: String,
    },
    Add {
        customer: String,
        amount: f64,
    },
    Delete {
        id: usize,
    },
    Edit {
        id: usize,
        customer: String,
        amount: f64,
    },
    Back,
    Exit,
}
