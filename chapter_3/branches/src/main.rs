fn main() {
    let number = 3;
    // if statements do not require parenthesis
    if number < 5 {
        println!("number is < 5");
    } else {
        println!("number is > 5");
    }
    // we can also use if as an expression (like in kotlin)
    let number = if number == 3 { 5 } else { 6 };
    println!("Number is now {}", number);
}
