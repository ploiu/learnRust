/*
From project euler

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/
fn main() {
    let multiples = determine_3_5_multiples(10);
    // add up all the multiples
    let mut sum = 0i32;
    for multiple in multiples.iter() {
        sum += multiple;
    }
    println!("sum: {}", sum);
}

fn determine_3_5_multiples(below: i32) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    // go through each number from 0 to 1000 and add it to list if it's a 3 or 5 multiple
    for num in 0..below {
        if num % 3 == 0 || num % 5 == 0 {
            list.push(num);
        }
    }
    list
}
