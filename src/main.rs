use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let mut cal_mode = String::new();
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Calculator!! (only intergers are supported right now)");
    println!("Choose your mode: ");
    println!("1. Addition, 2. Subtraction, 3. Multiplication, 4. Division");
    io::stdin()
        .read_line(&mut cal_mode)
        .expect("Failed to read line");
    let cal_mode: i32 = cal_mode.trim().parse().expect("Please type a number!");
    if cal_mode == 1 { // Addition
        println!("Enter your first number: ");
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");
        println!("Enter your second number: ");
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line");
        let input1: i32 = input1.trim().parse().expect("Please type a number!");
        let input2: i32 = input2.trim().parse().expect("Please type a number!");
        println!("The result is: {}", input1 + &input2);
    } else if cal_mode == 2 { // Subtraction
        println!("Enter your first number: ");
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");
        println!("Enter your second number: ");
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line");
        let input1: i32 = input1.trim().parse().expect("Please type a number!");
        let input2: i32 = input2.trim().parse().expect("Please type a number!");
        println!("The result is: {}", input1 - &input2);
    } else if cal_mode == 3 { // Multiplication
        println!("Enter your first number: ");
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");
        println!("Enter your second number: ");
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line");
        let input1: i32 = input1.trim().parse().expect("Please type a number!");
        let input2: i32 = input2.trim().parse().expect("Please type a number!");
        println!("The result is: {}", input1 * &input2);
    } else if cal_mode == 4 { // Division
        println!("Enter your first number: ");
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");
        println!("Enter your second number: ");
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line");
        let input1: i32 = input1.trim().parse().expect("Please type a number!");
        let input2: i32 = input2.trim().parse().expect("Please type a number!");
        println!("The result is: {}", input1 / &input2);
        if input1 % &input2 != 0 {
            println!("The remainder is: {}", input1 % &input2);
        }
    } else {
        println!("Please type numbers ranging from 1-4.")
    }
    println!("The program will be closed in 2 seconds.");
    thread::sleep(Duration::from_secs(2));
}
