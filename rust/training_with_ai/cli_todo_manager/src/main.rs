use std::{fmt, io::stdin, ops::RangeInclusive};

enum TaskStatus {
    Done,
    Postponed,
    Active,
}

impl TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Done => "Done".to_string(),
            TaskStatus::Postponed => "Postponed".to_string(),
            TaskStatus::Active => "Active".to_string(),
        }
    }
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
        write!(
            f,
            "title: {}, description: {}, status: {}",
            self.title.trim(),
            self.description.trim(),
            self.status.to_string()
        )
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

    'main_loop: loop {
        println!("What do you want to do?");

        let top_level_options = [
            "Add new Task",
            "Delete an existing task",
            "Edit an existing task",
            "See all tasks",
        ]
        .map(|s| s.to_string());

        top_level_options
            .iter()
            .enumerate()
            .for_each(|(i, op)| println!("{}. {}", i + 1, op));

        let user_input = get_user_input();

        while !validate_user_input_range(&user_input, 1..=user_input.len()) {
            println!("Enter an option between 1 and 4");
            continue 'main_loop;
        }

        match user_input.as_str() {
            "1" => add_a_task(&mut tasks),
            "2" => delete_a_task(&mut tasks),
            "3" => println!("Editing a task"),
            "4" => list_tasks(&tasks),
            _ => println!("Invalid input"),
        };
    }
}

fn add_a_task(tasks: &mut Vec<Task>) {
    println!("Enter title:");
    let title = get_user_input();
    println!("Enter description:");
    let description = get_user_input();

    let next_id = (tasks.len() + 1).to_string();

    let new_task = Task::new(next_id, title, description);
    tasks.push(new_task);
    println!("Task added successfully");
}

fn delete_a_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No task to delete");
        return;
    }

    tasks
        .iter()
        .enumerate()
        .for_each(|(i, Task { title, .. })| {
            println!("{}. {}", i + 1, title.trim());
        });

    println!("Enter the number of the task to delete:");
    let user_selection = get_user_input();

    match user_selection.parse::<usize>() {
        Ok(num) if (1..=tasks.len()).contains(&num) => {
            let removed_task = (*tasks).remove(num - 1);
            println!("Successfully deleted: {}", removed_task.title.trim());
        }
        _ => println!("{user_selection} Not a valid task number"),
    }
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("You haven't added any task yet");
        return;
    }

    tasks.iter().for_each(|t| println!("{}", t));
}

fn get_user_input() -> String {
    loop {
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => return input.trim().to_string(),
            Err(_) => {
                println!("A problem occurred, retry!");
                continue;
            }
        }
    }
}

fn validate_user_input_range(user_input: &String, range: RangeInclusive<usize>) -> bool {
    match user_input.parse::<usize>() {
        Ok(n) => range.contains(&n),
        Err(_) => false,
    }
}
