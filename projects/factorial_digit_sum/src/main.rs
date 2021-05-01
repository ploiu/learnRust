/*
from project euler

n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
*/
extern crate num;
// bigInt is required because the factorial of 100 exceeds the size a f64 can store
use num::bigint::Sign;
use num::BigInt;

fn main() {
    let fact = factorial(100);
    let digit_sum = sum_digits(fact);
    println!("{}", digit_sum);
}

fn factorial(number: i32) -> BigInt {
    let mut fact = BigInt::new(Sign::Plus, Vec::new()) + 1;
    // since multiplication doesn't matter on the order, we can just start from 1 and go to number
    for num in 1..number + 1 {
        // BigInt has the mul trait, which allows us to perform multiplication for all types it specifies
        fact *= num;
    }
    fact
}

fn sum_digits(number: BigInt) -> u32 {
    let number = number.to_string();
    let mut sum: u32 = 0;
    for num in number.chars() {
        match num.to_digit(10) {
            Some(number) => sum += number,
            None => (),
        }
    }
    sum
}
