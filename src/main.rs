mod save;
mod load;
mod search;

use dialoguer::Select;
use save::save;
use load::load;
use search::search;

fn main() {
    let selection = select();

    if selection == 0 {
        save();
    } else if selection == 1 {
        load();
    } else if selection == 2 {
        search();
    } 

    println!("Hello, world!");
}

fn select() -> usize {
    let opts = vec!["Save", "Load", "Search", "Exit"];
    let selection = Select::new()
        .with_prompt("Select an option")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap();

    println!("You selected: {}", opts[selection]);
    return selection;
}
