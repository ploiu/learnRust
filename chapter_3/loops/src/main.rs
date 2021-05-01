fn main() {
    // this creates an infinite loop
    loop {
        println!("again!");
        //... but the `break` keyword (like in most other languages I've learned) will exit the loop
        break;
    }
    // while loops are in rust too
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("AND WE HAVE LIFTOFF");
    // we can also use for loops to iterate over a collection
    let array = [0, 1, 2, 3, 4];
    for element in array.iter() {
        println!("The value is: {}", element);
    }
}
