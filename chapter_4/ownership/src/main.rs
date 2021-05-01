fn main() {
    // create a string with no known length at compile time (it's stored on the heap)
    let s = String::from("hello there");
    // create a string with a known length at compile time (it's stored on the stack I think)
    let s2 = "hi";
    take_ownership(s); // s has moved, so we can't reference it from this point on
    take_ownership(s2.to_string()); // becaue s2 was &str not string, we have to change it
    println!("s2 is still accessible: {}", s2); // since s2 didn't move, we can still access it

    let s3 = give_ownership(); // ownership is assigned to s3
    let s4 = take_and_give_ownership(s3); // ownership is removed from s3 and given to s4
    println!("I can use s4, but not s3: {}", s4);
    let length = does_not_take_ownership(&s4);
    println!("I can still use s4: {}, the length is {}", s4, length);
    let mut s5 = String::from("hi");
    // we can modify s5 without having to return a value
    change_string(&mut s5);
    // we can still reference s5 here because it was not moved
    println!("s5 is now {}", s5);
}

fn take_ownership(str: String) {
    // in here str has been moved since it does not have the `Copy` type
    println!("{}", str);
} // at this point, `str`'s memory has been freed

fn give_ownership() -> String {
    String::from("test") // the value is created and moved to the variable it's stored into
}

fn take_and_give_ownership(str: String) -> String {
    str // ownership was moved into the function, and moved back out to whatever variable this function call is assigned to
}

// because this takes a reference to a string and not the string itself, the string does not move and ownership does not change
fn does_not_take_ownership(str: &String) -> usize {
    // at this point, str is _borrowed_, and cannot be changed. uncommenting the below out will cause the code to not compile
    // str.push_str("test");
    str.len()
}

// this function takes a reference, but because we mark it as mutable, it can alter the string without having to change its ownership
fn change_string(str: &mut String) {
    str.push_str(" and hi");
}
