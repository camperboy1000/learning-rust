use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io;
use std::io::Write;
use std::ops::Sub;
use std::sync::Mutex;

static FIBONACCI_MAP: Mutex<BTreeMap<u128, u128>> = Mutex::new(BTreeMap::new());

fn fibonacci(n: &u128) -> u128 {
    match n.cmp(&2) {
        Ordering::Less => 0,
        Ordering::Equal => 1,
        Ordering::Greater => {
            let fibonacci_map = FIBONACCI_MAP
                .lock()
                .expect("Locking thread paniced when reading value!");

            match fibonacci_map.get(n) {
                Some(result) => result.clone(),
                None => {
                    drop(fibonacci_map);

                    let result1 = fibonacci(&n.sub(1));
                    let result2 = fibonacci(&n.sub(2));

                    let final_result = result1 + result2;
                    let mut fibonacci_map = FIBONACCI_MAP
                        .lock()
                        .expect("Locking thread paniced when setting value!");
                    fibonacci_map.insert(n.clone(), final_result.clone());

                    final_result
                }
            }
        }
    }
}

fn main() {
    println!("Let's calculate the Fibonacci series!");

    let n: u128 = loop {
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

    println!("Result: {}", fibonacci(&n));
}
