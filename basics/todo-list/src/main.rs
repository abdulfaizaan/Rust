use std::io;

fn main() {
    println!("Welcome to the To-Do List App!");
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("\nPlease choose an option:");
        println!("1. Add a task");
        println!("2. View tasks");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to Read");
        let choice = choice.trim(); // Fixed typo and shadowed the variable safely

        match choice {
            "1" => {
                println!("Enter Your task:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to Read"); // Fixed target variable
                let task = task.trim().to_string();
                 
                if !task.is_empty() {
                    println!("Task Added: {}", task);
                    tasks.push(task);
                } else {
                    println!("Error: Task cannot be empty.");
                }
            }
            "2" => {
                if tasks.is_empty() {
                    println!("List Empty");
                } else {
                    // Added parentheses around the index/value pair
                    for (i, task) in tasks.iter().enumerate() {
                        println!("{} {}", i + 1, task);
                    }
                }
            }
            "3" => {
                println!("Exiting App");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}