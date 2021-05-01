/*
from project euler

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

fn main() {
    let smallest = smallest_divisible(20);
    println!("{}", smallest);
}

fn smallest_divisible(max: u32) -> u32 {
    let mut smallest = 0u32;
    // we need to start iterating from 0, but we don't have to do as much processing since 3 and 7 will rule out a lot of divisibles
    loop {
        // smallest must be a multiple of max
        smallest += max;
        // if it's not divisible by at least 3 and 7, skip this number; used to speed things up since these numbers aren't divisible by much
        if smallest % 3 != 0 || smallest % 7 != 0 {
            continue;
        }
        // if this number is divisible by all, return it as it would be the first one to be so
        if is_divisible_by_all(smallest, max) {
            return smallest;
        }
    }
}

fn is_divisible_by_all(num: u32, max: u32) -> bool {
    for x in 1..max {
        if num % x != 0 {
            return false;
        }
    }
    true
}
