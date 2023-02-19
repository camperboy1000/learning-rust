use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn fibonacci(n: u32) -> u128 {
    match n.cmp(&2) {
        Ordering::Less => 0,
        Ordering::Equal => 1,
        Ordering::Greater => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Let's calculate the Fibonacci series!");

    let n: u32 = loop {
        print!("Provide the nth term to calculate: ");
        io::stdout().flush().expect("Failed to flush stdout!");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin!");

        match input.trim().parse() {
            Ok(n) => break n,
            Err(_) => println!("Please provide a valid number!"),
        };
    };

    println!("Result: {}", fibonacci(n));
}
