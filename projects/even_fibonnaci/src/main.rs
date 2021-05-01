/*
from project euler

Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:

1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

*/

fn main() {
    let fib_values = get_fib_values_below(4_000_000);
    let mut sum = 0;
    // now add up all the values
    for val in fib_values.iter() {
        if val % 2 == 0 {
            sum += val;
        }
    }
    println!("Value is {}", sum);
}

fn get_fib_values_below(limit: i32) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    // we will calculate fibonnaci in here instead of a different function
    let mut previous = 0;
    let mut current = 1;
    loop {
        let temp = current;
        current += previous;
        previous = temp;
        // make sure to check if we're at the limit
        if current <= limit {
            values.push(current);
        } else {
            break;
        }
    }
    values
}