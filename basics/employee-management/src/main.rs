use std::io;

struct Employee {
    id: u32,
    name: String,
    position: String,
}


fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop{
        println!("Employee Management System");
        println!("1. Add Employee");
        println!("2. View Employees");
        println!("3. Exit");
        println!("4.Edit Employee`");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed");
        let choice = choice.trim();


        match choice{
            "1"=>{
                let mut id = String::new();
                let mut name = String::new();
                let mut position = String::new();

                println!("Enter Employee ID:");
                io::stdin().read_line(&mut id).expect("Failed");
                let id: u32 = id.trim().parse().expect("Invalid ID");

                if employees.iter().any(|e| e.id == id){
                    println!("Employee with ID {} already exists. Please try again.", id);
                    continue;
                }

                
                println!("Enter Employee Name:");
                io::stdin().read_line(&mut name).expect("Failed");
                let name = name.trim().to_string();

                println!("Enter Employee Position:");
                io::stdin().read_line(&mut position).expect("Failed");
                let position = position.trim().to_string(); 

                println!("Employee added successfully!");
                employees.push(Employee{id, name, position});

            }

            "2"=>{
                println!("Employee List:");
                if employees.is_empty(){
                    println!("No employees found.");
                } else {
                for (i , employee) in employees.iter().enumerate(){
                    println!("{}. {} - {} - {}", i+1, employee.id, employee.name, employee.position);
                }
            }
            }
            "3"=>{
                println!("Exiting...");
                break;
            }
            "4"=>{
                println!("Enter Employee ID to edit:");

                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed");
                let id: u32 = id.trim().parse().expect("Invalid ID");
                let mut found = false;

                for employee in employees.iter_mut(){
                    if employee.id == id{

                    found = true;
                    let mut name = String::new();
                    let mut position = String::new();
                    println!("Enter new name: or press Enter to keep current name");
                    io::stdin().read_line(&mut name).expect("Failed");
                    let name = name.trim().to_string();
                    if !name.is_empty(){
                        employee.name = name;
                    }
                    else{
                        println!("Name unchanged.");
                    }
                    io::stdin().read_line(&mut position).expect("Failed");
                    let position = position.trim().to_string();
                    println!("Enter new position: or press Enter to keep current position");
                    if !position.is_empty(){
                        employee.position = position;
                    }
                    else{
                        println!("Position unchanged.");
                    }
                }
            }
        }

            _=>{
                println!("Invalid choice. Please try again.");  
            }
        }
    }

}
