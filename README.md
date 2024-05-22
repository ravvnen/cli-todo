# CLI Task Management Tool

A simple command-line tool for managing tasks, built with Rust. This tool allows you to add, list, and delete tasks easily from your terminal.

## Features

- **Add Tasks**: Add a new task with a description.
- **List Tasks**: Display all tasks with their status (completed or not completed).
- **Delete Tasks**: Remove a task by its ID.

## Requirements

- Rust (Install Rust from [rustup.rs](https://rustup.rs))

## Installation

1. **Clone the Repository**:

   ```sh
   git clone https://github.com/your-username/cli-task-manager.git
   cd cli-task-manager
   ```

2. **Build the Release Version**:

   ```sh
   cargo build --release
   ```

3. **Move the Binary to a Directory in Your PATH**:

   ```sh
   sudo mv target/release/cli-task-manager /usr/local/bin/todo
   ```

   Ensure the binary has executable permissions:

   ```sh
   sudo chmod +x /usr/local/bin/todo
   ```

4. **Verify the Installation**:

   ```sh
   which todo
   ```

   This should output `/usr/local/bin/todo`. You can now run the CLI tool using the `todo` command.

## Configuration

Create a `.env` file in the project root directory to set the file path for storing tasks:

```sh
FILE_PATH=./tasks/todo_list.txt
```

Ensure the tasks directory exists:

```sh
mkdir -p ./tasks
```

## Usage

### Add a Task

```sh
todo add "Buy milk"
```

### List All Tasks

```sh
todo list
```

### Delete a Task

```sh
todo delete 1
```

### Examples

1. **Add a Task**:

   ```sh
   todo add "Buy milk"
   ```

   Output:

   ```sh
   Task added: Buy milk
   ```

2. **List All Tasks**:

   ```sh
   todo list
   ```

   Output:

   ```sh
   1: Buy milk [not completed]
   ```

3. **Delete a Task**:

   ```sh
   todo delete 1
   ```

   Output:

   ```sh
   Task with ID 1 deleted
   ```

## Project Structure

- `src/main.rs`: The main entry point for the CLI application.
- `src/task.rs`: Contains the `Task` struct and functions for managing tasks.
- `.env`: Configuration file for setting the task file path.
- `tasks/todo_list.txt`: The default file path for storing tasks.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any suggestions or improvements.

## License

This project is licensed under the MIT License.

---

### Sample Commands for Developers

For developers who want to run the application without installing it globally, here are some sample commands:

1. **Run the Application**:

   ```sh
   cargo run -- add "Test task"
   ```

2. **Run the Tests**:

   ```sh
   cargo test
   ```

3. **Build the Application**:
   ```sh
   cargo build --release
   ```
