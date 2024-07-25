use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Calculator!! (only addition is supported right now)");
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
    println!("The program will be closed in 5 seconds.");
    thread::sleep(Duration::from_secs(5));
}
