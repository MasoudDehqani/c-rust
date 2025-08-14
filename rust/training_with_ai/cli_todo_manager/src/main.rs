use std::io::{Read, stdin};

enum TaskStatus {
    Done,
    Postponed,
    Active,
}

struct Task {
    id: String,
    title: String,
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(title: String, description: String) -> Self {
        Self {
            id: String::from("1"),
            title,
            description,
            status: TaskStatus::Active,
        }
    }
}

enum UserAction {
    Add(Task),
    Delete { id: String },
    EditTitle { id: String, new_title: String },
    EditDescription { id: String, new_description: String },
    EditStatus { id: String, new_status: TaskStatus },
    ListTasks,
}

fn main() {
    let tasks: Vec<Task> = Vec::new();

    // println!("What do you want to do?");
    // println!("1. Add new a task");
    // println!("2. Delete an existing task");
    // println!("3. Edit an existing task");
    // println!("4. See all tasks");

    let mut user_action_input = String::new();

    let valid_inputs = [
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
    ];

    println!("What do you want to do?");
    println!("1. Add new a task");
    println!("2. Delete an existing task");
    println!("3. Edit an existing task");
    println!("4. See all tasks");

    while !valid_inputs.contains(&user_action_input) {
        user_action_input = String::new();
        stdin()
            .read_line(&mut user_action_input)
            .expect("Failed to read line");
        user_action_input.pop();
        if !valid_inputs.contains(&user_action_input) {
            println!("Enter an option between 1 and 4");
        }
    }

    match user_action_input.as_str() {
        "1" => println!("Adding a new task"),
        "2" => println!("Deleting a task"),
        "3" => println!("Editing a task"),
        "4" => println!("Listing all tasks"),
        _ => println!("Invalid input"),
    };
}
