use serde::{Deserialize, Serialize};
//use std::fs:OpenOptions;
//use std::io::{BufReader, BufWriter};
//use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}
