// this is how a non-builtin library is referenced
extern crate rand;
// this is how rust handles imports
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // dunno why println uses an exclamation point...the book hasn't stated yet but I remember seeing something about "macros"
    println!("Guess the number dude");
    // generate the random number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // the loop keyword creates an infinite loop
    loop {
        println!("Please enter your guess");
        // mutable values are declared with the `mut` keyword, which is similar to how Kotlin handles collections (MutableList instead of List)
        let mut guess = String::new(); // I'm assuming this is a static constructor in a way, or the `::` denotes a method call that doesn't need to be called on an instance
        io::stdin() /* use the io library to read and directly modify `guess`. Here we are passing in guess's memory address(?)*/
        .read_line(&mut guess)
        .expect("Failed to read line");
        // we can shadow locally-declared variables, which I think can lead to some hard-to-read code
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // println handles string formatting as well
        println!("You guessed: {}", guess);
        // use a match statement to determine if the user guessed the correct number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // exit the loop (wow just like every other language I've seen)
                break;
            }
        }
    }
}
