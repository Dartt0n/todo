use colored::*;
use directories::ProjectDirs;
use std::fs::{create_dir_all, OpenOptions};
use std::io::prelude::Read;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process;

pub struct Todo {
    pub file: PathBuf,
    pub todo: Vec<String>,
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        // Create new project.
        let todo_project = ProjectDirs::from("", "Dartt0n", "td");
        let todo_project = todo_project.expect("Couldn't initialize project");
        // Get config directory. For Example, $HOME/.config/td on Linux
        let todo_config_dir = todo_project.config_dir();
        let todo_path = todo_config_dir.join("global.todo");
        if !todo_path.exists() {
            // Create directory and file if not exits
            create_dir_all(&todo_path).expect("Couldn't create config directory");
        }

        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .expect("Couldn't open the todofile");

        // Creates a new buf reader
        let mut buf_reader = BufReader::new(&todofile);

        // Empty String ready to be filled with TODOs
        let mut contents = String::new();

        // Loads "contents" string with data
        buf_reader.read_to_string(&mut contents).unwrap();

        // Splits contents of the TODO file into a todo vector
        let todo = contents.to_string().lines().map(str::to_string).collect();

        // Returns todo
        Ok(Self {
            file: todo_path,
            todo,
        })
    }

    // Prints every todo saved
    pub fn list(&self) {
        // This loop will repeat itself for each taks in TODO file
        if self.todo.len() > 0 {
            println!("TODO:")
        }
        for (number, task) in self.todo.iter().enumerate() {
            // Converts number into a bold string
            let number = (number + 1).to_string().bold();
            // Saves the symbol of current task
            let symbol = &task[..4];
            // Saves a task without a symbol
            let task = &task[4..];
            // Checks if the current task is completed or not...
            if symbol == "[*] " {
                // DONE
                // If the task is completed, then it prints it with a strikethrough
                println!("   {} {}", format!("<{}>", number).bold(), task.strikethrough());
            } else if symbol == "[ ] " {
                // NOT DONE
                // If the task is not completed yet, then it will print it as it is
                println!("   {} {}", format!("<{}>", number).bold(), task);
            }
        }
    }

    pub fn raw(&self, arg: &[String]) {
        if arg.len() > 1 {
            eprintln!("todo raw takes only 1 argument, not {}", arg.len())
        } else if arg.is_empty() {
            eprintln!("todo raw takes 1 argument. Choose `done` or `todo`)");
        } else {
            // This loop will repeat itself for each taks in TODO file
            for task in self.todo.iter() {
                if task.len() > 5 {
                    // Saves the symbol of current task
                    let symbol = &task[..4];
                    // Saves a task without a symbol
                    let task = &task[4..];

                    // Checks if the current task is completed or not...
                    if symbol == "[*] " && arg[0] == "done" {
                        // DONE
                        //If the task is completed, then it prints it with a strikethrough
                        println!("{}", task);
                    } else if symbol == "[ ] " && arg[0] == "todo" {
                        // NOT DONE

                        //If the task is not completed yet, then it will print it as it is
                        println!("{}", task);
                    }
                }
            }
        }
    }
    // Adds a new todo
    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }
        // Opens the TODO file with a permission to:
        let todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist
            .append(true) // b) append a line to it
            .open(&self.file)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(todofile);
        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            // Appends a new task/s to the file
            let line = format!("[ ] {}\n", arg);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    // Removes a task
    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo rm takes at least 1 argument");
            process::exit(1);
        }
        // Opens the TODO file with a permission to:
        let todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open(&self.file)
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&"done".to_string()) && &line[..4] == "[*] " {
                continue;
            }
            if args.contains(&(pos + 1).to_string()) {
                continue;
            }

            let line = format!("{}\n", line);

            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    // Sorts done tasks
    pub fn sort(&self) {
        // Creates a new empty string
        let newtodo: String;

        let mut todo = String::new();
        let mut done = String::new();

        for line in self.todo.iter() {
            if line.len() > 5 {
                if &line[..4] == "[ ] " {
                    let line = format!("{}\n", line);
                    todo.push_str(&line);
                } else if &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    done.push_str(&line);
                }
            }
        }

        newtodo = format!("{}{}", &todo, &done);
        // Opens the TODO file with a permission to:
        let mut todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open(&self.file)
            .expect("Couldn't open the todo file");

        // Writes contents of a newtodo variable into the TODO file
        todofile
            .write_all(newtodo.as_bytes())
            .expect("Error while trying to save the todofile");
    }

    pub fn done(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo done takes at least 1 argument");
            process::exit(1);
        }

        // Opens the TODO file with a permission to overwrite it
        let todofile = OpenOptions::new()
            .write(true)
            .open(&self.file)
            .expect("Couldn't open the todofile");
        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if line.len() > 5 {
                if args.contains(&(pos + 1).to_string()) {
                    if &line[..4] == "[ ] " {
                        let line = format!("[*] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("unable to write data");
                    } else if &line[..4] == "[*] " {
                        let line = format!("[ ] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("unable to write data");
                    }
                } else if &line[..4] == "[ ] " || &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    buffer
                        .write_all(line.as_bytes())
                        .expect("unable to write data");
                }
            }
        }
    }
}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";

pub fn help() {
    // For readability
    println!("{}", TODO_HELP);
}
