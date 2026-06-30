use dialoguer::Input;

pub fn crawl(history: &mut Vec<String>) {
    let input: String = Input::new()
        .with_prompt("Enter the URL to crawl")
        .show_default(false)
        .interact_text()
        .expect("Failed to read line");
    
    history.push(input.clone());
    println!("Crawling URL: {input}");
}