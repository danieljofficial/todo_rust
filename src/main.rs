extern crate todo;
use todo::{add_todo, create_file, read_terminal_args, Todo};


fn main() {
    let mut todos: Vec<Todo> = vec![];
    add_todo(&mut todos, String::from("Properly learn rust"), false);
   let args = read_terminal_args();
    create_file(args[1].clone());
}
