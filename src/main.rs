mod ui;

use std::error::Error;

fn main()  -> Result<(), Box<dyn Error>> {
    ui::main_window::start_ui();
    Ok(())
}


