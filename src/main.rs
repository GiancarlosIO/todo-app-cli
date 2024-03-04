use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::str::FromStr;
use std::{collections::HashMap, io::Read};
use std::{
    hash::Hash,
    io::{self, Stdout, Write},
};

use std::time::{Duration, Instant};

const DEBOUNCE_INTERVAL_MS: u64 = 50;

fn main() {
    let mut last_event_time = Instant::now();

    // let action = std::env::args().nth(1).expect("Please specify an action");
    // let item = std::env::args().nth(2).expect("Please specify an item");
    // let mut todo = Todo::new().expect("> Error to initialize the DB.");
    let mut todo = Todo {
        map: HashMap::new(),
    };
    todo.map.insert("todo-1".to_string(), false);
    todo.map.insert("todo-2".to_string(), false);
    todo.map.insert("todo-3".to_string(), false);
    todo.map.insert("todo-3".to_string(), false);
    todo.map.insert("todo-4".to_string(), false);
    todo.map.insert("todo-5".to_string(), false);

    // setup terminal
    let mut stdout = io::stdout();
    match terminal::enable_raw_mode() {
        Err(why) => eprintln!(
            "> An error occurred when trying to enable raw mode: {}",
            why
        ),
        Ok(_) => (),
    }

    match execute!(stdout, cursor::Hide) {
        Err(why) => eprintln!(
            "> An error occurred when trying to hide the cursor: {}",
            why
        ),
        Ok(_) => (),
    }

    let mut selected_index = 0;

    loop {
        eprintln!("Current index: {}", selected_index);
        // draw ui
        match draw_ui(&mut stdout, &todo.map, selected_index) {
            Err(why) => eprintln!("> Error to draw ui cli: {}", why),
            Ok(_) => (),
        }

        // handle user input
        if let Ok(event) = read() {
            match event {
                Event::Key(key_event) => {
                    if debounce_elapsed(&mut last_event_time) {
                        match key_event.code {
                            KeyCode::Char(' ') => {
                                todo.toggle_by_index(selected_index);
                                match todo.save() {
                                    Ok(_) => {}
                                    Err(why) => eprintln!("> Error to save todo: {}", why),
                                }
                            }
                            KeyCode::Esc | KeyCode::Char('q') => {
                                break;
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                selected_index =
                                    move_cursor(&todo.map, selected_index, Direction::Up);
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                selected_index =
                                    move_cursor(&todo.map, selected_index, Direction::Down);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        } else {
            eprintln!("> Error to read the user input");
        }

        // println!("Enter action (add, complete, delete) and item (or 'quit' to exit):");

        // let mut input = String::new();
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read line");

        // let trimmed_input = input.trim().to_lowercase();
        // let mut parts = trimmed_input.split_whitespace();

        // let action = match parts.next() {
        //     Some(a) => a,
        //     None => {
        //         println!("Invalid input. Please try again");
        //         continue;
        //     }
        // };

        // if action == "quit" {
        //     break;
        // }

        // let item = match parts.next() {
        //     Some(i) => i,
        //     None => {
        //         println!("Please specify an item");
        //         continue;
        //     }
        // };

        // match action {
        //     "add" => {
        //         todo.insert(item.to_string());
        //         match todo.save() {
        //             Ok(_) => println!("> Todo saved"),
        //             Err(why) => println!(">An error occurred: {}", why),
        //         }
        //     }
        //     "complete" => match todo.complete(&item.to_string()) {
        //         None => println!("> '{}' is not present in the list", item),
        //         Some(_) => match todo.save() {
        //             Ok(_) => println!("> Todo saved"),
        //             Err(why) => println!("> An error ocurred {}", why),
        //         },
        //     },
        //     "delete" => {
        //         todo.delete(&item.to_string());
        //         match todo.save() {
        //             Ok(_) => println!("> Todo saved"),
        //             Err(why) => println!("> An error has occurred: {}", why),
        //         }
        //     }
        //     _ => {
        //         println!("> Invalid action. Valid action are 'add', 'complete' and 'delete'")
        //     }
        // }
    }

    // println!("> Exiting todo list application");

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
// Function to check if debounce interval has elapsed since the last event
fn debounce_elapsed(last_event_time: &mut Instant) -> bool {
    let now = Instant::now();
    let elapsed = now.duration_since(*last_event_time);
    if elapsed >= Duration::from_millis(DEBOUNCE_INTERVAL_MS) {
        *last_event_time = now;
        true // Debounce interval has elapsed
    } else {
        false // Debounce interval has not elapsed
    }
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
            Some(v) => Some(*v = true),
            None => None,
        }
    }

    fn toggle_by_index(&mut self, index: usize) -> Option<()> {
        // let key = self.map.keys().nth(index).expect("Invalid index");
        // match self.map.get_mut(key) {
        //     Some(v) => Some(*v = !*v),
        //     None => None,
        // }
        if let Some(key) = self.map.keys().nth(index) {
            println!("Toggle todo {} - {}", index, key);
            if let Some(value) = self.map.get_mut(&key.to_owned()) {
                *value = !*value;
                return Some(());
            }
        }
        None
    }

    fn delete(&mut self, key: &String) {
        self.map.remove(key);
    }
}

enum Direction {
    Up,
    Down,
}

fn move_cursor(todos: &HashMap<String, bool>, current_index: usize, direction: Direction) -> usize {
    match direction {
        Direction::Up => {
            if current_index > 0 {
                current_index - 1
            } else {
                current_index
            }
        }
        Direction::Down => {
            if current_index < todos.len() - 1 {
                current_index + 1
            } else {
                current_index
            }
        }
    }
}

fn draw_ui(
    stdout: &mut Stdout,
    todos: &HashMap<String, bool>,
    selected_index: usize,
) -> Result<(), io::Error> {
    execute!(
        stdout,
        crossterm::terminal::Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
        crossterm::style::Print("Todo List\n"),
    )?;
    let mut i = 0;

    for (key, value) in todos {
        let checkbox = if *value { "(*)" } else { "( )" };
        let color = if i == selected_index {
            crossterm::style::Color::Green
        } else {
            crossterm::style::Color::White
        };

        crossterm::execute!(
            stdout,
            crossterm::style::SetForegroundColor(color),
            crossterm::style::Print(format!("{} {}\n", checkbox, key)),
            crossterm::style::ResetColor,
        )?;

        i += 1;
    }

    stdout.flush()?;
    Ok(())
}
