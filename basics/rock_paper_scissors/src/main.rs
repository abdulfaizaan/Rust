use std::io;
use rand::Rng;


fn main() {

    let choices = ["Rock","Paper","Scissors"];
    loop{
        let mut player = String::new();
        println!("Enter your choice (Rock, Paper, Scissors):");
        io::stdin()
        .read_line(&mut player)
        .expect("Failed to read line");

    let player = player.trim().to_lowercase();
    let mut rng = rand::thread_rng();

        let computer = choices[rng.gen_range(0..choices.len())];

        println!("You chose: {}", player);
        println!("Computer chose: {}", computer);

        match (player.as_str(), computer.to_lowercase().as_str()) {
            ("rock","scissors") => println!("You win!"),
            ("paper","rock") => println!("You win!"),
            ("scissors","paper") => println!("You win!"),

            ("rock","paper") => println!("Computer wins!"),
            ("paper","scissors") => println!("Computer wins!"),
            ("scissors","rock") => println!("Computer wins!"),
            
            
            ("rock", "rock") => println!("It's a tie!"),
            ("paper", "paper") => println!("It's a tie!"),
            ("scissors", "scissors") => println!("It's a tie!"),


            _ => println!("Invalid choice! Please enter Rock, Paper, or Scissors."),

        }

        println!("Do you want to play again? (yes/no):");
        let mut play_again = String::new();
        io::stdin()
        .read_line(&mut play_again)
        .expect("Failed to read line"); 

        if play_again.trim().to_lowercase() != "yes" {
            break;
        } 
    }
    println!("Thanks for playing!");

}
