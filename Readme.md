# 🦀 My Rust Learning Journey

Welcome to my space for tracking my progress as I dive into the world of **Rust**! This repository serves as a digital dev-log for my daily learning, experimental code snippets, deep dives, and upcoming projects.

---

## 🚀 The Ultimate Goal
> "To master the art of ownership, achieve compile-time nirvana, and build blazing-fast, memory-safe systems."

*   **Systems Proficiency:** Write safe, concurrent, and highly optimized code without a garbage collector.
*   **WebAssembly & Beyond:** Bridge the gap between low-level performance and web ecosystems.
*   **Production-Ready CLI:** Build highly interactive, beautifully styled, and robust command-line tools.

---

## 📈 Current Status & Milestones

| Milestone | Status | Description | Completed Date |
| :--- | :---: | :--- | :---: |
| **01. The Basics** | ✅ | Variables, data types, control flow, and functions. | *June 2026* |
| **02. The Core** | 🦀 | Mastered **Ownership, Borrowing, and Mutability**. | *June 11, 2026* |
| **03. Advanced Concepts** | ⏳ | Lifetimes, Traits, Generics, and Smart Pointers. | *Up Next* |
| **04. Concurrency** | ⏳ | Fearless concurrency, threads, and message passing. | *Planned* |

---

## 🛠️ The Project Pipeline

Here are the projects I am building (and optimizing) to cement my understanding of the language:

### 1. 🧮 CLI Calculator
*   **Focus:** Pattern matching, robust error handling, and standard I/O parsing.
*   **Status:** `Completed `

### 2. 🪨 Rock, Paper, Scissors App
*   **Focus:**  `match` control flow, and building interactive terminal loops.
*   **Status:** `completed`

### 3. 📝 CLI Todo-List App
*   **Focus:** Structs, vectors, and basic file I/O to persist data locally.
*   **Status:** `Completed`

### 4. 💼 Employee Management System
*   **Focus:** Structs, vectors, and basic file I/O to persist data locally.
*   **Status:** `Completed`

---

## 📚 Resources & Toolkit
*   [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
*   [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
*   **IDE Setup:** VS Code + `rust-analyzer`

---

```Rust
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
```

<p align="center">
  Made with ❤️ and 🦀 by <b>Abdul Faizaan</b>
</p>