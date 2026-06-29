use std::io;

pub fn search() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    println!("Searching for: {input}");
}