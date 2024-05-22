mod task;

use dotenv::dotenv;
use task::{add_task, list_tasks, delete_task};

use crate::task::delete_tasks;

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
        "del" => 
        {
            if args.len() < 3 {
                eprintln!("Usage: {} del <task_id>", args[0]);
                return;
            }

            let description = args[2].clone();
            println!(": {}", description);

            if description == "all"{
                delete_tasks();
            }
            else {
                delete_task(description.parse().unwrap());
            
            }
        },    

        "list" => list_tasks(),
        _ => eprintln!("Unknown command: {}", command),
    }
}




