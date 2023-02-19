use std::collections::HashMap;

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let length: i32 = match numbers.len().try_into() {
        Ok(len) => len,
        Err(e) => {
            println!("{e}");
            return 0;
        }
    };

    for i in numbers {
        sum += i;
    }

    sum / length
}

fn median(numbers: &Vec<i32>) -> i32 {
    let length = numbers.len();
    let index = (length / 2) - 1;
    let mut sorted = numbers.clone();
    sorted.sort();

    match length % 2 == 0 {
        true => (sorted[index] + sorted[index + 1]) / 2,
        false => sorted[index],
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut hits = HashMap::new();
    let mut result = 0;
    let mut result_hits = 0;

    for i in numbers {
        let entry = hits.entry(i).or_insert(0);
        *entry += 1;
    }

    for (key, value) in hits {
        match result_hits < value {
            true => {
                result_hits = value;
                result = *key;
            }
            false => continue,
        }
    }

    result
}

fn main() {
    let numbers = Vec::from([-21, 9, 5, 7, -5, -21]);

    let mean_result = mean(&numbers);
    let median_result = median(&numbers);
    let mode_result = mode(&numbers);

    println!("Mean: {mean_result}");
    println!("Median: {median_result}");
    println!("Mode: {mode_result}");
}
