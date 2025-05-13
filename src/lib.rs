use std::{env, fs::File};

#[derive(Debug)]
pub struct Todo {
    description: String,
    is_completed: bool
}

impl Todo {
    pub fn new(description: String, is_completed: bool) -> Self {
        Todo { is_completed, description }
    }
}


pub fn add_todo(todos: &mut Vec<Todo>, description: String, is_completed: bool) -> &Vec<Todo> {

    todos.push(Todo::new(description, is_completed));
    todos
}

pub fn read_terminal_args() -> Vec<String>{
    let args = env::args().collect();
    println!("args: {:?}", args);
    args
}

pub fn create_file(filename: String) -> bool {
    let mut file = File::open(filename).expect("file not found");
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tempfile::
    use std::fs;

    #[test]
    fn todo_was_added() {
        let mut todos: Vec<Todo> = vec![];
        add_todo(&mut todos, String::from("test"), false);
        // let case = Todo { description: String::from("test"), is_completed: false  };
        // todos.contains();
        // println!("Todos: {:?}", todos);
        assert!(todos.iter().any(|todo| todo.description == "test"));
    }

    #[test]
    fn terminal_args_were_read() {
        let test_arg = String::from("/home/daniel/Documents/Programs/Rust_projects/todo/target/debug/deps/todo-a28ddf7bd49c804e");
        let expected_result = vec![test_arg];
        let actual_result: Vec<String> = read_terminal_args();
        assert_eq!(expected_result, actual_result);
    }
    #[test]
    fn tasks_file_was_created_if_nonexistent() {
        let filename = String::from("output.txt");
        let is_created: bool = create_file(filename);
    }
    // #[test]
    // fn todo_was_written_to_file() {

    // }
}