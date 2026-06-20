use std::{fs::{File, read_to_string}, io::{self, Error, Read, Result, Write}, path::Path};

use serde::{Deserialize, Serialize};




#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Todos{
    todos: Vec<String>,
    marks: Vec<bool>,
}


// impl Clone for Todos{
//     fn clone(&self) -> Todos {
//         Todos{
//             todos: self.todos.clone(),
//             marks: self.marks.clone(),
//         }
//     }
// }

impl Todos{
    pub fn new() -> Todos{
        Todos{
            todos: Vec::<String>::new(),
            marks: Vec::<bool>::new(),
        }
    }



    pub fn add(&mut self, text: String) -> Result<()>{
        self.todos.push(text);
        self.marks.push(false);
        Ok(())
    }

     

    pub fn list_format(self) -> Vec<String>{
        let mut list: Vec<String> = Vec::new();
        for i in 0..self.todos.len(){
            let mut formated = String::new();
            formated+=format!("{}<{}> - {}\n",
                i+1,
                if self.marks[i] {"*"} else {" "},
                self.todos[i])
            .as_str();
            list.push(formated);
        }
        list
    }



    pub fn text_format(self) -> String{
        let mut formated = String::new();
        for i in 0..self.todos.len(){
            formated+=format!("{}<{}> - {}\n",
                i+1,
                if self.marks[i] {"*"} else {" "},
                self.todos[i])
            .as_str();
        }
        formated
    }

    pub fn load(&mut self, filename: &str, mode: i32) -> Result<()>{
        let mut f = File::open(filename)?;
        let mut text = String::new();
        match f.read_to_string(&mut text) {
            Err(why) => panic!("couldn't read {}", why),
            Ok(_) => print!(""),
        }
        let data: Self = serde_json::from_str(&text).unwrap();

        *self = data;

        Ok(())
    }

    pub fn save(&mut self, filename: &str) -> Result<()>{
        let j = serde_json::to_string(&self)?;
        
        let mut f = File::create(filename)?;
        let _ = f.write_all(j.as_bytes())?;

        Ok(())
    }

    fn remove(&mut self, index: usize){
        self.todos.remove(index);
        self.marks.remove(index);
    }

}





pub fn main() {
    let todos_filename: &str = "todos.txt";
    let mut todos = Todos::new();
    let _ = todos.load(todos_filename, 0);
    
    let _ = todos.add("smth".to_string());

    todos.remove(2);

    let _ = todos.save(todos_filename);

    println!("{}", todos.text_format());

    
}

   
