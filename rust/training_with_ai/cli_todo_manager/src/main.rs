use std::{fmt, io::stdin};

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
    fn new(id: String, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            status: TaskStatus::Active,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.title, self.description)
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
    let mut tasks: Vec<Task> = Vec::new();

    let mut user_action_input = String::new();

    let valid_inputs = [
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
    ];

    loop {
        println!("What do you want to do?");
        println!("1. Add new a task");
        println!("2. Delete an existing task");
        println!("3. Edit an existing task");
        println!("4. See all tasks");

        while !valid_inputs.contains(&user_action_input) {
            reset_user_input(&mut user_action_input);
            stdin()
                .read_line(&mut user_action_input)
                .expect("Failed to read line");
            user_action_input.pop();
            if !valid_inputs.contains(&user_action_input) {
                println!("Enter an option between 1 and 4");
            }
        }

        match user_action_input.as_str() {
            "1" => add_a_task(&mut tasks, &mut user_action_input),
            "2" => {
                delete_a_task(&mut tasks);
                reset_user_input(&mut user_action_input);
            }
            "3" => println!("Editing a task"),
            "4" => {
                tasks.iter().for_each(|t| println!("{}", t));
                reset_user_input(&mut user_action_input);
            }
            _ => println!("Invalid input"),
        };
    }
}

fn reset_user_input(input: &mut String) {
    *input = String::new();
}

fn add_a_task(tasks: &mut Vec<Task>, user_action_input: &mut String) {
    let mut title = String::new();
    let mut desc = String::new();
    println!("Enter title:");
    stdin().read_line(&mut title).expect("Failed to read");
    println!("Enter description:");
    stdin().read_line(&mut desc).expect("Failed to read");

    let next_id = tasks
        .last()
        .map(|t| t.id.clone())
        .unwrap_or("1".to_string());

    let new_task = Task::new(next_id, title, desc);
    tasks.push(new_task);
    reset_user_input(user_action_input);
}

fn delete_a_task(tasks: &mut Vec<Task>) {
    tasks.iter().enumerate().for_each(|(i, t)| {
        println!("{}. {}", i + 1, t.title);
    });

    println!("Enter the number of the task to delete:");
    let mut user_selection = String::new();
    let tasks_len = tasks.len();

    stdin()
        .read_line(&mut user_selection)
        .expect("Failed to read");

    user_selection.pop();
    let user_selection: usize = user_selection.parse().unwrap_or(0);

    if !(1..=tasks_len).contains(&user_selection) {
        println!("Not a task number");
        return;
    }

    (*tasks).remove(user_selection - 1);
}
