extern crate todo;
use todo::{add_todo, ensure_file_exists, load_todos_from_file_path, read_terminal_args, save_todos_to_file, write_todo_to_file, Todo};


fn main() {
    // let mut todos: Vec<Todo> = vec![];
    let args = read_terminal_args();
    let filename = args[1].clone();
    let task = args[2].clone();
    // add_todo(&mut todos, task, false);
    // save_todos_to_file(&filename, &todos).unwrap();
    let _ = write_todo_to_file(&filename, task, false);
    let loaded_todos = load_todos_from_file_path(&filename).unwrap();
    println!("Todos: {:#?}", loaded_todos);
}
