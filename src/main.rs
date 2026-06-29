mod save;
mod load;
mod search;
mod manage;

use dialoguer::Select;
use save::save;
use load::load;
use search::search;
use manage::manage;

fn main() {
    let selection = select();

    if selection == 0 {
        save();
    } else if selection == 1 {
        load();
    } else if selection == 2 {
        search();
    } else if selection == 3 {
        manage();
    }

}

fn select() -> usize {
    let opts = vec!["Save", "Load", "Search", "Manage", "Exit"];
    let selection = Select::new()
        .with_prompt("Select an option")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap();

    println!("You selected: {}", opts[selection]);
    return selection;
}
