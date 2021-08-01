fn main() {
    // as we've seen before, this is how you create a String
    let s = String::new();
    // you can also call `to_string` on anything that has the `Display` trait
    let s = "test".to_string();
    // we can also use string from
    let mut s = String::from("test");
    // there are a few ways to update a string
    s.push_str(" test2");
    s += " hi";
    s = format!("hello, {}", "world");
    println!("{}", s);
    s.push('!');
    // the addition operator can take ownership
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    // the add function takes ownership of self and takes a reference to the rest of the parameters
    let s3 = s1 + &s2;
    // s1 was moved, so we can't use it anymore (but we can use s2 since we passed by reference)
    println!("s3 is {} and s2 is {}", s3, s2);

    /*
        Strings can't be indexed because of how UTF-8 works. Strings are actually wrappers for
        Vec<u8>, meaning that indexing into them will return a byte, and not necessarily the
        grapheme we intend to get.

        On top of this, if we create a string slice that would slice into the middle of a
        UTF-8 character, the program will panic and crash.Err

        Also, indexing does not guarantee O(1) time
    */
    let s = String::from("नमस्ते"); // Hindi for hello, each character is actually 2 bytes, and some have diacritics
                                  // we can use an iterator to get all the chars
    for c in s.chars() {
        println!("{}", c);
    }

    println!("{}", s);
}
