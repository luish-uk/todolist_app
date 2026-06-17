
use std::io;

mod main_txt;
mod main_json;

fn main() {
    let mut var = String::new();
    println!("Choose a mode (txt or json)?");
    io::stdin().read_line(&mut var);
    let var = var.trim();
    if var == "txt" {
        main_txt::main();
    } else if var == "json" {
        main_json::main();
    } else {
        println!("Sorry, incorrect input, please enter txt or json");
    }
}