use serde::{Deserialize, Serialize};

const TODO_FILE: &str = "todo.json";

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn load_todos() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let data = match std::fs::read_to_string(TODO_FILE) {
        Ok(content) => content,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Ok(vec![]);
            } else {
                return Err(e.into());
            }
        }
    };

    if data.trim().is_empty() {
        return Ok(vec![]);
    }

    let todos: Vec<Todo> = serde_json::from_str(&data)?;

    Ok(todos)
}

fn save_todos(todos: &[Todo]) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(&todos)?;
    std::fs::write(TODO_FILE, data)?;
    Ok(())
}

fn add_todo(title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut todos = load_todos()?;
    let new_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let new_todo = Todo {
        id: new_id,
        title: title.to_string(),
        completed: false,
    };
    todos.push(new_todo);
    save_todos(&todos)?;
    Ok(())
}

fn set_done(id: &u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut todos: Vec<Todo> = load_todos()?;
    for todo in &mut todos {
        if todo.id == *id {
            todo.completed = true;
        }
    }
    save_todos(&todos)?;
    Ok(())
}

fn main() {
    println!("Hello, world!");
    add_todo("Learn Rust 2").expect("Failed to add todo");
    set_done(&2).expect("Failed to set todo as done");  
    match load_todos() {
        Ok(todos) => {
            for todo in todos {
                println!(
                    "ID: {}, Title: {}, Completed: {}",
                    todo.id, todo.title, todo.completed
                );
            }
        }
        Err(e) => eprintln!("Error loading todos: {}", e),
    }
}
