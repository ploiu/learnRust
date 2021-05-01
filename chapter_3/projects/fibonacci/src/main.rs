use std::io;

fn main() {
    loop {
        println!("enter how many times to fibonacci");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("This should never happen");
        // parse it
        let number: u64 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("the fibonacci of {} is {}", number, fibonacci(number));
    }
}

/**
 * this is a scuffed fibonacci implementation. Very inefficient but I haven't
 * yet learned how to handle this efficiently. It works though
 */
fn fibonacci(n: u64) -> u64 {
    let mut current: u64 = 1;
    let mut previous: u64 = 0;
    let mut count: u64 = 0;
    while count < n {
        let temp = current;
        current += previous;
        previous = temp;
        count += 1;
    }
    return current;
}
