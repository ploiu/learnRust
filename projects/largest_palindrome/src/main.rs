/*
from project euler

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn main() {
    /*
    We could start at 100 for each one and use a nested loop to figure this out, and therefore brute force it
    That would take way too long since it would have to go through 899 * 899 = 808201 times
    Since we're just trying to go for the largest palindrome created from 3-digit numbers, we can choose a starting point
    and just hope we hit a palindrome while not taking up as much time
    */
    let starting_point = 900;
    let palindrome = find_palindrome_for(starting_point);
    println!("{}", palindrome);
}

fn find_palindrome_for(starting_point: u32) -> u32 {
    let mut current_product: u32 = 0;
    let mut highest_palindrome = 0;
    for first in starting_point..999 {
        // save time since we already know the products of everything before this point in the outer loop
        for second in first..999 {
            current_product = first as u32 * second as u32;
            if is_palindrome(current_product) && current_product > highest_palindrome {
                highest_palindrome = current_product;
            }
        }
    }
    highest_palindrome
}

fn is_palindrome(product: u32) -> bool {
    let stringified: String = product.to_string();
    stringified == stringified.chars().rev().collect::<String>()
}
