fn main() {
    println!("Calling from the main function");
    with_parameters(20);
    statements_and_expressions();
    return_value();
}

fn with_parameters(x: i32) {
    println!("The value of x is {}", x);
}

fn statements_and_expressions() {
    // we can create a block of code to do some cool things
    let y = {
        let x = 32;
        // expressions do not end in a semicolon, or else they will not return anything
        x + 1
    };

    println!("the value of y is {}", y);
}

fn return_value() -> i32 {
    // don't have to specify the return keyword, but you can if you want
    5
}
