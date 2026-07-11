mod save;
mod load;
mod search;
mod manage;
mod crawl;
mod init_db;

use dialoguer::Select;
use save::save;
use load::load;
use search::search;
use manage::manage;
use crawl::crawl;
use init_db::init_db;

fn main() {

    // Init db if the user selects so -> public/private db
    init_db();

    let mut history: Vec<String> = Vec::new();
    
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
            manage(&mut history);
        } else if selection == 4 {
            history.push("Crawl".to_string());
            crawl(&mut history);
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
