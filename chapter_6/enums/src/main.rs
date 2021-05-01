// it appears that enums in rust are used for more use cases and in a slightly different way from what I'm used to
enum IpAddressType {
    // we can also store data in an enum instance
    V4(String),
    V6(String),
}

// enum instances can take different values, and an enum can be used to group similar types together
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

// enums can also have methods defined
impl Message {
    fn call(&self) {
        // we can decide what to do depending on what type we are
    }
}

fn main() {
    // enums are referenced with the association operator or whatever you call the double colon
    let localhost = IpAddressType::V4(String::from("127.0.0.1"));
    let loopback = IpAddressType::V6(String::from("::1"));
    get_ip_address(&localhost);
    get_ip_address(&loopback);
    // rust does not have a concept of nulls, but it has an enum that can be used for when a value is absent for some reason or another
    let absent_number: Option<i32> = None;
    // match can be done with option in order to do some things
    let none = add_one(absent_number);
    let six = add_one(Some(5));
    // match also has something similar to the default branch in a switch statement
    let value = 0u8;
    match value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // this line is a catch all, and should always be put after all other cases
        _ => (), // this is the unit type, but I don't know what it can do
    };
    // we can also use if..let to do shorthand matches in cases where we only care about one value
    let some_number = Some(3u8);
    // we don't have to specify u8 here becase it infers from the other variable what you need. Trying to change `Some(5)` to a number larger than 255 will not compile
    if let Some(5) = some_number {
        println!("three!");
    } else {
        // this branch is the same as the `_` arm in a match expression
        println!("not three");
    }
}

fn add_one(number: Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(num) => Some(num + 1),
    }
}

fn get_ip_address(address: &IpAddressType) {
    match address {
        IpAddressType::V4(address) => println!("IPV4 address: {}", address),
        IpAddressType::V6(address) => println!("IPV6 address: {}", address),
    };
}
