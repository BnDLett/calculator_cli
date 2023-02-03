use std::io;

fn main() {
    loop {
        println!("\nPlease choose any one of the operations: add, subtract, multiply, divide or exit to close the program.\n");
        let operation: String = input().to_lowercase();

        if operation.trim() == "exit".to_string() {
            break;
        }

        if operation.trim() == "add".to_string() {
            println!("{}", add());
        }
        if operation.trim() == "subtract".to_string() {
            println!("{}", subtract());
        }
        if operation.trim() == "multiply".to_string() {
            println!("{}", multiply());
        }
        if operation.trim() == "divide".to_string() {
            println!("{}", divide());
        }
    }
}

fn input() -> String { // I'm doing this since I simply don't like the long way of doing inputs. TL;DR: Preference issue.
    let mut to_return = String::new();
    io::stdin().read_line(&mut to_return).unwrap();
    return to_return;
}

fn add() -> i32 {
    println!("Please enter a value for x:");
    let x: i32 = input().trim().parse::<i32>().unwrap_or(0);

    println!("Please enter a value for y:");
    let y: i32 = input().trim().parse::<i32>().unwrap_or(0);

    let result: i32 = x + y;
    
    print!("\x1B[2J\x1B[1;1H");
    return result;
}

fn subtract() -> i32 {
    println!("Please enter a value for x:");
    let x: i32 = input().trim().parse::<i32>().unwrap_or(0);

    println!("Please enter a value for y:");
    let y: i32 = input().trim().parse::<i32>().unwrap_or(0);

    let result: i32 = x - y;
    
    print!("\x1B[2J\x1B[1;1H");
    return result;
}

fn multiply() -> i32 {
    println!("Please enter a value for x:");
    let x: i32 = input().trim().parse::<i32>().unwrap_or(0);

    println!("Please enter a value for y:");
    let y: i32 = input().trim().parse::<i32>().unwrap_or(0);

    let result: i32 = x * y;
    
    print!("\x1B[2J\x1B[1;1H");
    return result;
}

fn divide() -> i32 {
    println!("Please enter a value for x:");
    let x: i32 = input().trim().parse::<i32>().unwrap_or(0);

    println!("Please enter a value for y:");
    let y: i32 = input().trim().parse::<i32>().unwrap_or(0);

    let result: i32 = x / y;
    
    print!("\x1B[2J\x1B[1;1H");
    return result;
}
