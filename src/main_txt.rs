use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::io::BufReader;
use std::env;


pub fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).cloned().unwrap_or(String::new());
    //let query = args.get(2).cloned().unwrap_or(String::new());
    let mut setup: bool = false;
    let file_creation = init();
    match file_creation {
        Ok(()) => println!("Created new file"),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("File already exists");
                setup = true;
            }
        }
    }
    if setup {
        println!("Reading todolist file contents");
    } else {
        println!("Creating todolist file!");
    }
    if command == "add" {
        for x in 2..(args.len()) {
            let query = args.get(x).cloned().unwrap_or(String::new());
            new_item(&query);
        }
    } else if command == "view" {
        println!("\n\nTodo List:");
        read();
    } else if command == "del" {
        for x in 2..(args.len()) {
            let query = args.get(x).cloned().unwrap_or(String::new());
            delete(query);
        }
    } else if command == "done" {
        for x in 2..(args.len()) {
            let query = args.get(x).cloned().unwrap_or(String::new());
            complete(query);
        }
    } else if command == "help" {
        help();
    }

}

fn init() -> std::io::Result<()> {
    let mut file = File::create_new("todolist.txt")?;
    file.write_all(b"")?;
    Ok(())
}

fn read() -> std::io::Result<()> {
    let file = File::open("todolist.txt")?;
    let completed_file = File::open("completed.txt")?;
    let mut buf_reader: BufReader<File> = BufReader::new(file);
    let mut buf_reader_completed: BufReader<File> = BufReader::new(completed_file);
    let mut contents = String::new();
    let mut contents_completed = String::new();
    buf_reader.read_to_string(&mut contents)?;
    buf_reader_completed.read_to_string(&mut contents_completed)?;
    //assert_eq!(contents, "Laundry (Example)");
    println!("{}", contents);
    println!("Completed: \n{}", contents_completed);
    Ok(())
}

fn new_item(item: &str) -> std::io::Result<()> {
    let mut f: File = File::options().append(true).open("todolist.txt")?;
    writeln!(&mut f, "{}", item)?;
    Ok(())
}

fn delete(item: String) -> std::io::Result<()> {
    let mut contents= convert_contents().unwrap();
    let mut newfile = File::create("todolist.txt")?;
    if contents.contains(&item) {
        let filtered: Vec<&str> = contents.lines().filter(|line| !line.contains(&item)).collect();
        contents = filtered.join("\n");
        contents.push_str("\n");
        contents = contents.trim_start().to_string();
    } else {
        println!("Task not found");
    };
    let mut bytes = contents.as_bytes().to_vec();
    newfile.write_all(&bytes)?;
    Ok(())
}

fn convert_contents() -> std::io::Result<(String)> {
    let mut bytes = std::fs::read("todolist.txt")?;
    let mut contents = String::from_utf8(bytes.clone()).unwrap();
    Ok((contents))
}

fn complete(item: String) -> std::io::Result<()> {
    let mut contents= convert_contents().unwrap();
    let mut f: File = File::options().append(true).open("completed.txt")?;
    if contents.contains(&item) {
        let filtered: Vec<&str> = contents.lines().filter(|line| line.contains(&item)).collect();
        contents = filtered.join("\n");
        contents.push_str("\n");
        let mut bytes = contents.as_bytes().to_vec();
        f.write_all(&bytes)?;
        delete(item)?;
    } else {
        println!("Task not found");
    }
    Ok(())
}

fn help() {
    println!("Usage:\ncargo run [command] [value]\n\nCommands:\nadd, view, del, done, help\n\nValues: can be one or multiple, not applicable for view command");
}
