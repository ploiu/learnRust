// constants can be declared anywhere, and it's not frowned upon to put them in the global scope if necessar
const CONST_VALUE: u16 = 5_000;

fn main() {
    println!("CONST_VALUE: {}", CONST_VALUE);
    // by default a value is immutable when declared
    let x = 6;
    println!("x is {}", x);
    // ... but with the `mut` keyword we can change it and even reassign to it later
    let mut y = "5";
    y = "42";
    let y: i32 = match y.parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    println!("y after parsing is {}", y);
    data_types();
    compound_types();
}

fn data_types() {
    // integer types
    let byte: u8 = 255;
    let short: u16 = 65_535;
    let int: u32 = 4_294_967_295;
    // there is also a 64-bit integer, but I don't feel like typing that out

    // not specifiying a type defaults to i32, which is the fastest type
    let defaultInt = 100;
}

fn compound_types() {
    // rust has tuples and arrays as primitive types, which is pretty cool
    let tuple: (i32, i64, char) = (500, 27, 'h');
    // we can also pattern match and destructure a tuple
    let (x, y, z) = tuple;
    println!("x = {}; y = {}; z = {}", x, y, z);
    // accessing a tuple's elements makes sense but is a little different from what I'm used to
    println!("Tuple first element: {}", tuple.0);

    // arrays are pretty straightforward as well; it also looks like the size is a part of the type
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // array elements are accessed in the way you'd expect
    println!("first array item: {}", array[0]);
}
