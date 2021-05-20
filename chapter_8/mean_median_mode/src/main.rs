use std::collections::HashMap;
use std::io;

fn main() {
    let mut numbers = get_inputs();
    println!("Got inputs: {:?}", numbers);
    let mean = determine_mean(&numbers);
    let median = determine_median(&mut numbers);
    let mode = determine_mode(&numbers);
    println!("mean: [{}], median: [{}], mode: [{}]", mean, median, mode);
}

fn get_inputs() -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let input = io::stdin();
    loop {
        println!("Enter an i32, or an empty line to exit the loop");
        let mut line = String::new();
        input
            .read_line(&mut line)
            .expect("this should never happen");
        match line.trim().parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => {
                if line.trim() != "" {
                    println!("{} is not a number. To quit entering numbers, press enter without typing anything", line);
                }
            }
        }
        if line == "\n" || line == "\r\n" {
            break;
        }
    }
    return numbers;
}

fn determine_mean(numbers: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for num in numbers.iter() {
        sum += num;
    }
    return sum as f64 / numbers.len() as f64;
}

fn determine_median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort_unstable();
    if numbers.len() == 0 {
        return 0;
    }
    let middle = numbers.len() / 2;
    // we can guarantee that there is a middle value
    return numbers[middle];
}

fn determine_mode(numbers: &Vec<i32>) -> i32 {
    let mut num_count = HashMap::<i32, i32>::new();
    for num in numbers.iter() {
        match num_count.get(num) {
            // need to put in blocks here or else it will try and use it as a return value
            Some(_) => {
                num_count.entry(*num).and_modify(|it| *it += 1);
            }
            None => {
                num_count.insert(*num, 1);
            }
        }
    }
    // this compares entries, not values like I thought it would
    *num_count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0
}
