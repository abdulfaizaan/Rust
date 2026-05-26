use std::io;

fn main() {
    loop {
        println!("====== Calculator ======");

        let mut number1 = String::new();

        println!("Enter first number:");

        io::stdin()
            .read_line(&mut number1)
            .expect("Failed");

        let number1: f64 = number1.trim().parse().expect("Invalid number");

        let mut number2 = String::new();

        println!("Enter second number:");

        io::stdin()
            .read_line(&mut number2)
            .expect("Failed");

        let number2: f64 = number2.trim().parse().expect("Invalid number");

        let mut operator = String::new();

        println!("Enter operator (+, -, *, /):");

        io::stdin()
            .read_line(&mut operator)
            .expect("Failed");

        let operator = operator.trim();
        
            // if operator == "+" {
    //     let result = add(number1, number2);

    //     println!(
    //         "The sum of {} and {} is: {}",
    //         number1,
    //         number2,
    //         result
    //     );
    // } else if operator == "-" {
    //     let result = sub(number1, number2);

    //     println!(
    //         "The difference of {} and {} is: {}",
    //         number1,
    //         number2,
    //         result
    //     );
    // } else if operator == "*" {
    //     let result = mul(number1, number2);

    //     println!(
    //         "The multiplication of {} and {} is: {}",
    //         number1,
    //         number2,
    //         result
    //     );
    // } else if operator == "/" {
    //     let result = div(number1, number2);

    //     println!(
    //         "The division of {} and {} is: {}",
    //         number1,
    //         number2,
    //         result
    //     );
    // } else {
    //     println!("Invalid Operator");
    // }

        let result = match operator {
            "+" => add(number1, number2),
            "-" => sub(number1, number2),
            "*" => mul(number1, number2),
            "/" => div(number1, number2),
            _ => {
                println!("Invalid operator");
                continue;
            }
        };

        println!("Result: {}", result);

        // Ask to continue
        let mut choice = String::new();

        println!("Do you want to continue? (y/n)");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed");

        if choice.trim() == "n" {
            break;
        }
    }

    


    println!("Calculator closed");



}



fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn sub(a: f64, b: f64) -> f64 {
    a - b
}

fn mul(a: f64, b: f64) -> f64 {
    a * b
}

fn div(a: f64, b: f64) -> f64 {
    a / b
}