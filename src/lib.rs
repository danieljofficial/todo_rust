use core::fmt;
use std::{env, fs::{File, OpenOptions}, io::{self, Read, Write}, path::Path};

#[derive(Debug)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(PartialEq)]
pub struct Todo {
    description: String,
    is_completed: bool
}

impl Todo {
    pub fn new(description: String, is_completed: bool) -> Self {
        Todo { is_completed, description }
    }

    pub fn toggle_completion(&mut self) {
        self.is_completed = !self.is_completed;
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_is_completed(&self) -> bool {
        self.is_completed
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.is_completed {"[✓]"} else {"[ ]"};
        write!(f, "{} {}", status, self.description)
    }
}


pub fn add_todo(todos: &mut Vec<Todo>, description: String, is_completed: bool) -> &Vec<Todo> {

    todos.push(Todo::new(description, is_completed));
    todos
}

pub type TodoResult<T> = Result<T, io::Error>;

pub fn save_todos_to_file( filename: &str, todos: &[Todo]) -> TodoResult<()>{
    ensure_file_exists(filename)?;

    let serialized = serde_json::to_string_pretty(todos)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)?;
        
    file.write_all(serialized.as_bytes())?;

    Ok(())
    
}

pub fn load_todos_from_file_path(filename: &str) -> TodoResult<Vec<Todo>> {
    ensure_file_exists(filename)?;

    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    let todos = serde_json::from_str(&contents)?;
    Ok(todos)
}

pub fn read_terminal_args() -> Vec<String>{
    let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", args);
    if args.len() > 3 {
        panic!("Arguments must not exceed three");
    }
    args
}

pub fn ensure_file_exists(filename: &str) -> TodoResult<()> {
    let path = Path::new(filename);

    if !path.exists() {
        File::create(path)?;
    }

    Ok(())
}

pub fn write_todo_to_file(filename: &str, description: String, is_completed: bool) -> TodoResult<()> {
    ensure_file_exists(filename)?;

    let mut todos = load_todos_from_file_path(filename)?;

    todos.push(Todo::new(description, is_completed));

    save_todos_to_file(filename, &todos)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn todo_was_added() {
        let mut todos: Vec<Todo> = vec![];
        add_todo(&mut todos, String::from("test"), false);
        assert!(todos.iter().any(|todo| todo.description == "test"));
    }

    #[test] 
    fn todo_display_formats_correctly () {
       let todo_incomplete = Todo::new(String::from("test task"), false); 
       let todo_complete = Todo::new(String::from("completed task"), true); 

       assert_eq!(todo_incomplete.to_string(), "[ ] test task");
       assert_eq!(todo_complete.to_string(), "[✓] completed task");
    }

    #[test]
    pub fn todo_can_toggle_completion () {
        let mut todo = Todo::new(String::from("test task"), false);

        assert_eq!(todo.get_is_completed(), false);

        todo.toggle_completion();

        assert_eq!(todo.get_is_completed(), true);

        todo.toggle_completion();
        assert_eq!(todo.get_is_completed(), false);
    }  

    #[test]
    fn todo_saved_and_loaded_correctly() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let todos = vec![Todo::new(String::from("task 1"), false), Todo::new(String::from("task 2"), true)];

        save_todos_to_file(&path, &todos).unwrap();

        let loaded_todos = load_todos_from_file_path(&path).unwrap();
        println!("Todos: {:?}", &todos);

        assert_eq!( todos, loaded_todos);
    } 

    #[test] 
    fn write_single_todo_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        write_todo_to_file(&path, String::from("single task"), false).unwrap();
        
        let todos = load_todos_from_file_path(&path).unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].description, String::from("single task"));
        assert_eq!(todos[0].is_completed, false);
    }
}