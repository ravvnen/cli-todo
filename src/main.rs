mod task;

use dotenv::dotenv;
use std::fs;
use std::fs::File;
//use serde::{Deserialize, Serialize};
use std::env;
use std::io::Read;
use std::io::Write;
use std::env::args;


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
            add_task(description);
            
            
        },
        "list" => list_tasks(),
        _ => eprintln!("Unknown command: {}", command),
    }
}

pub fn add_task(description: String) {

    // Create a file in the local file system
    let mut data_file = File::create("todo_list.txt").expect("Unable to create file");

    // Write a string to the file
    data_file.write_all(description.as_bytes()).expect("Unable to write data");

    println!("Task added");
}

fn list_tasks() {
    // Read a file in the local file system
    let mut data_file = File::open("todo_list.txt").expect("Unable to open file");

    // Read the file contents into a string
    let mut contents = String::new();

    data_file.read_to_string(&mut contents).unwrap();
    println!("Tasks: {}", contents);
}




