use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

const SLEEP_DURATION: Duration = Duration::from_secs(3);
const CONVERSION_CONSTANT: f32 = 9.0 / 5.0;

fn main() {
    let mode = loop {
        clearscreen::clear().expect("Failed to clear the screen!");

        println!("What type of conversion would you like to perform?");
        println!("\t[1] Fahrenheit to Celsius");
        println!("\t[2] Celsius to Fahrenheit");
        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout!");

        let mut mode = String::new();

        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read from stdin!");
        mode = mode.trim().to_string();

        if mode == "1" || mode == "2" {
            break mode;
        }

        println!("Please select a valid option...");
        sleep(SLEEP_DURATION);
    };

    let tempurature: f32 = loop {
        print!("Enter a tempurature: ");
        io::stdout().flush().expect("Failtd to flush stdout!");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin!");

        match input.trim().parse() {
            Ok(tempurature) => break tempurature,
            Err(_) => {
                println!("Please enter a valid number...");
                sleep(SLEEP_DURATION);
            }
        }
    };

    let conversion: f32 = if mode == "1" {
        (tempurature - 32.0) * (1.0 / CONVERSION_CONSTANT)
    } else {
        (tempurature * CONVERSION_CONSTANT) + 32.0
    };

    println!("Converted tempurature: {}", conversion)
}
