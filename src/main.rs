use chrono;
use crossterm::{
    cursor,
    event::{self, Event, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use std::{
    fmt::Display,
    io::{stdout, Stdout},
    time::{Duration, Instant, SystemTime},
};

fn main() {
    let mut last_event_time = Instant::now();
    let mut app = App::init();

    app.add_todo("My new todo".to_string());
    app.add_todo("Todo 2".to_string());

    let mut stdout = stdout();
    match terminal::enable_raw_mode() {
        Err(why) => eprintln!(
            "> An error ocurred when trying to enable the raw mode: {}",
            why
        ),
        Ok(_) => (),
    }
    match execute!(stdout, cursor::Hide) {
        Err(why) => eprintln!("> An error ocurred when trying to hide the cursor: {}", why),
        Ok(_) => (),
    }

    let mut cursor_line_index = 0;

    loop {
        let mut cursor_limit = 0;
        match app.state {
            State::Initial => {
                cursor_limit = 1;
            }
            _ => {}
        }
        println!("cursor: {}, limit: {}", cursor_line_index, cursor_limit);
        // Draw UI
        match app.state {
            State::Initial => {
                execute!(
                    stdout,
                    crossterm::terminal::Clear(ClearType::All),
                    crossterm::cursor::MoveTo(0, 0),
                    crossterm::style::Print(
                        "Welcome! Ready to be more productive? 🚀\n
What do you want to do?\
"
                    )
                )
                .expect("> Error to show the initial page.");

                // execute!(stdout, crossterm::style::Print("\n() Show all todos"),)
                //     .expect("> Error to render commands");
                print_to_terminal(
                    &mut stdout,
                    "\n - Show all todos".to_string(),
                    cursor_line_index == 0,
                );
                print_to_terminal(
                    &mut stdout,
                    "\n - Add a new todo".to_string(),
                    cursor_line_index == 1,
                );
                print_to_terminal(
                    &mut stdout,
                    "\n\nPress 'ctrl+c' or 'q' to exit.".to_string(),
                    false,
                );
            }
            State::ShowTodos => {
                clear_and_print(
                    &mut stdout,
                    "There is the full information about you todos 👇:".to_string(),
                    false,
                );
                for todo in app.todos.iter() {
                    print_to_terminal(&mut stdout, todo.get_info(), true);
                }
                print_to_terminal(
                    &mut stdout,
                    "\n\n> Press 'escape' or 'q' key to return to the menu.".to_string(),
                    false,
                )
            }
            State::AddTodo => {}
        }

        // event key handlers
        if let Ok(event) = crossterm::event::read() {
            match event {
                // close the app when the user press ctrl+c
                Event::Key(crossterm::event::KeyEvent {
                    modifiers: crossterm::event::KeyModifiers::CONTROL,
                    code,
                    ..
                }) => match code {
                    crossterm::event::KeyCode::Char('c') => {
                        println!("\n\n> Exiting application... 👋");
                        match execute!(stdout, cursor::Show) {
                            Err(why) => {
                                eprintln!("> Error when trying to show the cursor: {}", why)
                            }
                            Ok(_) => {}
                        }
                        std::process::exit(0);
                    }
                    _ => {}
                },
                crossterm::event::Event::Key(key_event) => {
                    if debounce_elapsed(&mut last_event_time) {
                        match key_event.code {
                            crossterm::event::KeyCode::Char('q')
                            | crossterm::event::KeyCode::Esc => match app.state {
                                State::Initial => {
                                    match execute!(stdout, crossterm::cursor::Show) {
                                        Err(why) => {
                                            println!(
                                                "> Error when trying to show the cursor: {}",
                                                why
                                            )
                                        }
                                        Ok(_) => {}
                                    }
                                    println!("\n\n> Exiting application... 👋");
                                    break;
                                }
                                // go back if we are not in the initial page
                                _ => app.state = State::Initial,
                            },
                            crossterm::event::KeyCode::Up
                            | crossterm::event::KeyCode::Char('k') => {
                                cursor_line_index =
                                    move_cursor(cursor_line_index, cursor_limit, Direction::Up);
                            }
                            crossterm::event::KeyCode::Down
                            | crossterm::event::KeyCode::Char('j') => {
                                cursor_line_index =
                                    move_cursor(cursor_line_index, cursor_limit, Direction::Down);
                            }
                            crossterm::event::KeyCode::Enter => match app.state {
                                State::Initial => {
                                    if cursor_line_index == 0 {
                                        app.state = State::ShowTodos;
                                    }
                                    if cursor_line_index == 1 {
                                        app.state = State::AddTodo;
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

struct Todo {
    id: String,
    title: String,
    done: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
enum State {
    Initial,
    ShowTodos,
    AddTodo,
}

struct App {
    todos: Vec<Todo>,
    state: State,
}

impl App {
    fn init() -> App {
        App {
            todos: Vec::new(),
            state: State::Initial,
        }
    }

    fn add_todo(&mut self, title: String) {
        let id = generate_id();
        let created_at = chrono::Utc::now();

        self.todos.push(Todo {
            id,
            title,
            done: false,
            created_at,
        })
    }
}

impl Todo {
    fn get_info(&self) -> String {
        let local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(self.created_at);
        let formatted = local.format("%d %B %Y %I:%M%P");
        format!(
            "\nid: {}\ntitle: {}\ndone: {}\ncreated_at: {}\n",
            self.id, self.title, self.done, formatted
        )
    }
}

fn generate_id() -> String {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos()
        .to_string()
}

// Function to check if debounce interval has elapsed since the last event
const DEBOUNCE_INTERVAL_MS: u64 = 50;
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

fn print_to_terminal(stdout: &mut Stdout, t: String, selected: bool) {
    let color = if selected {
        crossterm::style::Color::Green
    } else {
        crossterm::style::Color::White
    };
    crossterm::execute!(
        stdout,
        crossterm::style::SetForegroundColor(color),
        crossterm::style::Print(t),
        crossterm::style::ResetColor,
    )
    .expect("> An error ocurred when trying to print_to_terminal to terminal.");
}

enum Direction {
    Up,
    Down,
}
fn move_cursor(current: i32, limit: i32, direction: Direction) -> i32 {
    match direction {
        Direction::Down => {
            if current < limit {
                current + 1
            } else {
                current
            }
        }
        Direction::Up => {
            if current > 0 {
                current - 1
            } else {
                current
            }
        }
    }
}

fn clear_and_print(stdout: &mut Stdout, t: String, selected: bool) {
    execute!(
        stdout,
        crossterm::terminal::Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
    )
    .expect("> Error to show the initial page.");
    print_to_terminal(stdout, t, selected)
}
