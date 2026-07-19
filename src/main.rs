mod save;
mod load;
mod search;
mod manage;
mod crawl;
mod init_db;
mod log_in;

use dialoguer::Select;
use save::save;
use load::load;
use search::search;
use manage::manage;
use crawl::crawl;
use init_db::init_db;
use log_in::log_in;

fn main() {
    let mut history: Vec<String> = Vec::new();

    let log_choice = Select::new()
        .with_prompt("Do you want to log in?")
        .items(&["Yes", "No"])
        .default(0)
        .interact()
        .expect("Failed to read selection");

    // Init db
    let conn = init_db(log_choice == 0, &mut history).unwrap();

    // Log in if the user wants to log in
    let mut logged_in: bool = log_choice == 0 && log_in(&mut history, &conn);

    loop {
        let selection = select();
        
        if selection == 0 {
            history.push("Save".to_string());
            save(&mut history);
        } else if selection == 1 {
            history.push("Load".to_string());
            load(&mut history);
        } else if selection == 2 {
            history.push("Search".to_string());
            search(&mut history);
        } else if selection == 3 {
            history.push("Manage".to_string());
            manage(&mut history, &conn, &mut logged_in);
        } else if selection == 4 {
            history.push("Crawl".to_string());
            crawl(&mut history, &conn);
        } else {
            println!("Exiting...");
            history.push("Exit".to_string());
            return;
        }
    }
}

fn select() -> usize {
    let opts = vec!["Save", "Load", "Search", "Manage", "Crawl", "Exit"];
    let selection = Select::new()
        .with_prompt("Select an option")
        .items(&opts)
        .default(0)
        .interact()
        .unwrap();

    println!("You selected: {}", opts[selection]);
    return selection;
}
