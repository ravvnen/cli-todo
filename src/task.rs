use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use std::fs::File;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: usize,
    description: String,
    //completed: bool,
}

impl Task {
    pub fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            //completed: false,
        }
    }

    // pub fn complete(&mut self) {
    //     self.completed = true;
    // }
}

pub fn load_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();

    // Attempt to open the file in read mode
    if let Ok(mut data_file) = OpenOptions::new().read(true).open("todo_list.txt") {
        let mut contents = String::new();
        data_file.read_to_string(&mut contents).unwrap();
        tasks = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
    }

    tasks
}

pub fn add_task(description: String) {

    // Load tasks from the file
    let mut tasks: Vec<Task> = load_tasks();
    let id: usize = tasks.len() + 1;

    // Create a new task object
    let task = Task::new(id, description);

    // Append the new task to the list of tasks
    tasks.push(task);
    
    // Serialize the tasks to a JSON string
    let serialized_tasks = serde_json::to_string_pretty(&tasks).expect("Unable to serialize tasks");
    
    
    // Open TODO-file in write mode
   // Open the file in write mode and overwrite it with the updated tasks list
   let mut data_file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .create(true)
    .open("todo_list.txt")
    .expect("Unable to open file");

    data_file.write_all(serialized_tasks.as_bytes()).expect("Unable to write file");
   
}

pub fn list_tasks() {
   let tasks = load_tasks();
   for task in tasks {
       println!("{}: {}", task.id, task.description);
   }
   
}

pub fn delete_task(id: usize) {
    let mut tasks = load_tasks();


    tasks.retain(|task| task.id != id);

    let serialized_tasks = serde_json::to_string_pretty(&tasks).expect("Unable to serialize tasks");

    let mut data_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("todo_list.txt")
        .expect("Unable to open file"); 

    data_file.write_all(serialized_tasks.as_bytes()).expect("Unable to write file");

    println!("Task {} deleted", id)
}