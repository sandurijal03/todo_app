mod processes;
mod state;
mod todo;

use processes::process_input;
use serde_json::{json, value::Value, Map};
use state::{read_file, write_to_file};
use std::env;

use todo::todo_factory;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json".to_string());

    let status: String = match &state.get(title) {
        Some(result) => result.to_string().replace('\"', ""),
        None => "pending".to_string(),
    };

    state.insert(title.to_string(), json!(status));
    write_to_file(String::from("./state.json"), &mut state);

    let item = todo_factory(status, title.to_string()).expect("Cannot be empty");

    process_input(item, command.to_string(), &state);
}
