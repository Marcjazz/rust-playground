use std::{borrow::Borrow, usize};

use io::Error;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
enum PriorityLevel {
    UrgentAndImportant,
    NotUrgentButImportant,
    NotImportantButUrgent,
    NotImportantAndNotUrgent,
}

impl PriorityLevel {
    fn from(priority: u8) -> Result<Self, Error> {
        match priority {
            4 => Ok(PriorityLevel::UrgentAndImportant),
            3 => Ok(PriorityLevel::NotUrgentButImportant),
            2 => Ok(PriorityLevel::NotImportantButUrgent),
            1 => Ok(PriorityLevel::NotImportantAndNotUrgent),
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                "Priority level goes from 1 to 4",
            )),
        }
    }
}

#[derive(Debug, Clone)]
struct TaskOptional {
    is_done: bool,
    title: Option<String>,
    priority_level: Option<PriorityLevel>,
}

#[derive(Debug, Clone)]
struct Task {
    title: String,
    priority_level: PriorityLevel,
    is_done: bool,
}

#[derive(Debug)]
struct UserBoard {
    username: String,
    tasks: Vec<Task>,
    number_of_done_tasks: usize,
    number_of_high_important_tasks: u8,
}

impl UserBoard {
    fn new(username: &str) -> UserBoard {
        UserBoard {
            tasks: Vec::new(),
            number_of_done_tasks: 0,
            username: username.to_string(),
            number_of_high_important_tasks: 0,
        }
    }

    fn find_tasks(&self, is_done: bool, priority_level: PriorityLevel) -> Vec<Task> {
        let mut tasks: Vec<Task> = Vec::new();
        for task in &self.tasks {
            if task.is_done == is_done || task.priority_level == priority_level {
                tasks.push(task.clone());
            }
        }

        tasks
    }

    fn create_task(&mut self, title: &str, priority_level: PriorityLevel) -> Task {
        match priority_level {
            PriorityLevel::UrgentAndImportant => {
                self.number_of_high_important_tasks += 1;
            }
            _ => {}
        }

        let new_task = Task {
            is_done: false,
            priority_level,
            title: title.to_string(),
        };
        self.tasks.push(new_task.clone());
        new_task
    }

    fn update_task(&mut self, index: usize, data: TaskOptional) -> Result<(), Error> {
        let mut number_of_done_tasks: usize = 0;
        let mut number_of_high_important_tasks: u8 = 0;

        for i in 0..self.tasks.len() {
            let task = self
                .tasks
                .get_mut(i)
                .expect(format!("Failed to retrieve mutable task at index {}", i).as_str());
            if i == index {
                number_of_done_tasks += if data.is_done { 1 } else { 0 };
                match data
                    .priority_level
                    .as_ref()
                    .unwrap_or(PriorityLevel::NotImportantAndNotUrgent.borrow())
                {
                    PriorityLevel::UrgentAndImportant => {
                        if self.number_of_high_important_tasks == 255 {
                            Err(Error::new(
                                io::ErrorKind::OutOfMemory,
                                "You have reached your limit of hign important tasks",
                            ))?
                        }
                        number_of_high_important_tasks += 1;
                    }
                    _ => {}
                }

                task.title = data.title.unwrap_or(task.title.clone());
                task.is_done = data.is_done;
                task.priority_level = data.priority_level.unwrap_or(task.priority_level.clone());
                break;
            }
        }

        self.number_of_done_tasks = number_of_done_tasks;
        self.number_of_high_important_tasks = number_of_high_important_tasks;

        Ok(())
    }

    fn remove_task(&mut self, index: usize) -> Task {
        let task = self.tasks.remove(index);

        if task.is_done {
            self.number_of_done_tasks -= 1;
        }

        match task.priority_level {
            PriorityLevel::UrgentAndImportant => {
                self.number_of_high_important_tasks -= 1;
            }
            _ => {}
        }

        task
    }
}

#[derive(Debug)]
enum Operation {
    Create,
    Update,
    Retrieve,
    Remove,
}

impl Operation {
    fn from(method: &str) -> Result<Self, Error> {
        match method {
            "create" => Ok(Operation::Create),
            "update" => Ok(Operation::Update),
            "retrieve" => Ok(Operation::Retrieve),
            "remove" => Ok(Operation::Remove),
            _ => Err(Error::new(io::ErrorKind::InvalidInput, "Invalid method")),
        }
    }
}

pub fn todo_list() {
    println!("Welcome to your Rusty Todo list!!");
    println!("To get starting, give us your username: ");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to username");
    let mut user_board = UserBoard::new(&username.trim());

    println!("{}", "\nAllow operations:".green());
    println!("1. Create new task (cmd: create <task_title> <task_priority>)");
    println!("2. Update existing task title (cmd: update <task_id> <task_title> <task_priority> <is_done>)");
    println!("3. Find existing tasks (cmd: find <is_done> <task_priority>");
    println!("4. Remove existing task (cmd: remove task <task_id>)");
    println!("5. Exit program (cmd: exit)");
    loop {
        println!("{}", format!("\nOverview ({})", user_board.username).blue());
        println!(
            "{}",
            format!(
                "Number of completed taks: {}\t\tNumber of high priority tasks: {}",
                user_board.number_of_done_tasks, user_board.number_of_high_important_tasks
            )
            .blue()
        );
        println!("> Type your comaand line operation below\n> ");

        let mut command_string = String::new();
        io::stdin()
            .read_line(&mut command_string)
            .expect("Failed to command line");

        if command_string.eq("exit") {
            break;
        }

        let mut args: Vec<String> = Vec::new();
        let mut method: Option<Operation> = Option::None;

        let split_whitespace = command_string.split_whitespace();
        split_whitespace.for_each(|x| {
            if method.is_none() {
                method = Some(Operation::from(x).expect("Invalid operation!"));
            } else {
                args.push(x.to_string())
            }
        });

        if method.is_none() {
            println!("Please provide a valid command!")
        }

        match method.unwrap() {
            Operation::Create => {
                let title = args.get(0).expect("title is required");
                let priority = args.get(1).expect("task priority is required");
                let priority_level = match priority.trim().parse() {
                    Ok(p) => PriorityLevel::from(p).unwrap(),
                    Err(_) => panic!("task priority must be a number between 1 and 5"),
                };
                let task = user_board.create_task(&title, priority_level);
                println!(
                    "{}",
                    format!("Successfully created new task {:?}", task).green()
                )
            }
            Operation::Update => {
                let task_index: usize = args
                    .get(0)
                    .expect("task index is required")
                    .trim()
                    .parse()
                    .expect("tasl index must be a u8 integer");

                let done_arg = args.get(1).expect("is_done params must be provided");
                let is_done = match done_arg.as_str() {
                    "true" | "1" => true,
                    "false" | "0" => false,
                    _ => {
                        eprintln!("Invalid boolean argument: {}", done_arg);
                        return;
                    }
                };

                let priority = args.get(2);
                let mut priority_level: Option<PriorityLevel> = Option::None;
                if priority.is_some() {
                    priority_level = Some(match priority.unwrap().trim().parse() {
                        Ok(p) => PriorityLevel::from(p).unwrap(),
                        Err(_) => {
                            eprintln!("task priority must be a number between 1 and 5");
                            return;
                        }
                    });
                }

                let title = match args.get(3) {
                    Some(title) => Some(title.to_string()),
                    None => None,
                };

                match user_board.update_task(
                    task_index,
                    TaskOptional {
                        title,
                        is_done,
                        priority_level,
                    },
                ) {
                    Ok(_) => println!(
                        "{}",
                        format!("Successfully updated task {}", task_index).green()
                    ),
                    Err(err) => {
                        eprintln!("Failed to update task {}: {}", task_index, err.to_string())
                    }
                }
            }
            Operation::Retrieve => {
                let done_arg = args.get(0).expect("is_done params must be provided");
                let is_done = match done_arg.as_str() {
                    "true" | "1" => true,
                    "false" | "0" => false,
                    _ => {
                        eprintln!("Invalid boolean argument: {}", done_arg);
                        return;
                    }
                };
                let priority = args.get(1).expect("task priority is required");
                let priority_level = match priority.trim().parse() {
                    Ok(p) => PriorityLevel::from(p).unwrap(),
                    Err(_) => panic!("task priority must be a number between 1 and 5"),
                };
                println!(
                    "{}",
                    format!(
                        "Successfully retrieved tasks: \n\t{:?}",
                        user_board.find_tasks(is_done, priority_level)
                    )
                    .green()
                )
            }
            Operation::Remove => {
                let task_index: usize = args
                    .get(0)
                    .expect("task index is required")
                    .trim()
                    .parse()
                    .expect("tasl index must be a u8 integer");

                println!(
                    "{}",
                    format!(
                        "Successfully deleted task: \n{:?}",
                        user_board.remove_task(task_index)
                    )
                    .green()
                )
            }
        }
    }
}
