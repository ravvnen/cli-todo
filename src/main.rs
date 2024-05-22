mod task;

use dotenv::dotenv;
use std::fs;
use std::fs::File;
//use serde::{Deserialize, Serialize};
use std::env;
use std::io::Read;
use std::io::Write;
use std::env::args;
use std::fs::OpenOptions;



fn main() {
    dotenv().ok();
    let args: Vec<String> = std::env::args().collect();
    
    //let path = env::var("PATH").expect("PATH must be set");
    //println!("Tasks file: {}", path);


    if args.len() < 2 {
        eprintln!("Usage: {} <command> [<args>]", args[0]);
        return;
    }   

    let command = &args[1];
    println!("Command: {}", command);

    match command.as_str() {
        "add" => 
        {
            if args.len() < 3 {
                eprintln!("Usage: {} add <task_description>", args[0]);
                return;
            }
            let description = args[2].clone();
            println!("Task description: {}", description);

            if description == "" {
                eprintln!("Task description cannot be empty");
                return;
            }
            add_task(description);
            
            
        },
        "list" => list_tasks(),
        _ => eprintln!("Unknown command: {}", command),
    }
}

pub fn add_task(description: String) {

    // Open a file in write mode
    let mut data_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("todo_list.txt")
        .expect("Unable to create file");

    // Add a new line to the file
    data_file.write(b"\n").expect("Unable to write data");

    // Write a string to the file
    data_file.write(description.as_bytes()).expect("Unable to write data");

    println!("Task added: {}", description);
}

fn list_tasks() {
    // Read a file in the local file system
    let mut data_file = OpenOptions::new()
        .read(true)
        .open("todo_list.txt")
        .expect("Unable to open file");

    // Read the file contents into a string
    let mut contents = String::new();

    data_file.read_to_string(&mut contents).unwrap();
    println!("Tasks: {}", contents);
}




