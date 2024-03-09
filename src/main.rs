use chrono;
use std::time::SystemTime;

fn main() {
    let mut app = App::init();

    app.add_todo("My new todo".to_string());

    for todo in app.todos {
        todo.log();
    }
}

struct Todo {
    id: String,
    title: String,
    done: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

struct App {
    todos: Vec<Todo>,
}

impl App {
    fn init() -> App {
        App { todos: Vec::new() }
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
    fn log(self) {
        let local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(self.created_at);
        let formatted = local.format("%d %B %Y %I:%M%P");
        println!(
            "\nid: {}\ntitle: {}\ndone: {}\ncreated_at: {}\n",
            self.id, self.title, self.done, formatted
        );
    }
}

fn generate_id() -> String {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos()
        .to_string()
}
