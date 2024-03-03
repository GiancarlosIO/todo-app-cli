use std::io;
use std::str::FromStr;
use std::{collections::HashMap, io::Read};

fn main() {
    // let action = std::env::args().nth(1).expect("Please specify an action");
    // let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new().expect("> Error to initialize the DB.");

    loop {
        println!("Enter action (add, complete, delete) and item (or 'quit' to exit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim().to_lowercase();
        let mut parts = trimmed_input.split_whitespace();

        let action = match parts.next() {
            Some(a) => a,
            None => {
                println!("Invalid input. Please try again");
                continue;
            }
        };

        if action == "quit" {
            break;
        }

        let item = match parts.next() {
            Some(i) => i,
            None => {
                println!("Please specify an item");
                continue;
            }
        };

        match action {
            "add" => {
                todo.insert(item.to_string());
                match todo.save() {
                    Ok(_) => println!("> Todo saved"),
                    Err(why) => println!(">An error occurred: {}", why),
                }
            }
            "complete" => match todo.complete(&item.to_string()) {
                None => println!("> '{}' is not present in the list", item),
                Some(_) => match todo.save() {
                    Ok(_) => println!("> Todo saved"),
                    Err(why) => println!("> An error ocurred {}", why),
                },
            },
            "delete" => {
                todo.delete(&item.to_string());
                match todo.save() {
                    Ok(_) => println!("> Todo saved"),
                    Err(why) => println!("> An error has occurred: {}", why),
                }
            }
            _ => {
                println!("> Invalid action. Valid action are 'add', 'complete' and 'delete'")
            }
        }
    }

    println!("> Exiting todo list application");

    // if action == "add" {
    //     todo.insert(item);
    //     match todo.save() {
    //         Ok(_) => println!("> Todo saved"),
    //         Err(why) => println!("An error ocurred: {}", why),
    //     }
    // } else if action == "complete" {
    //     match todo.complete(&item) {
    //         None => println!("> '{}' is not present in the list", item),
    //         Some(_) => match todo.save() {
    //             Ok(_) => println!("> Todo saved"),
    //             Err(why) => println!("> An error occurred {}", why),
    //         },
    //     }
    // }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass  true as value
        self.map.insert(key, true);
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in &self.map {
            let record = format!("{}\t{}\n", k, v);
            content.insert_str(0, &record);
        }
        std::fs::write("db.txt", content)
    }

    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();

        match f.read_to_string(&mut content) {
            Ok(_) => println!("> Success to convert to string"),
            Err(why) => println!("> An error ocurred {}", why),
        }

        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();

        Ok(Todo { map })
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn delete(&mut self, key: &String) {
        self.map.remove(key);
    }
}
