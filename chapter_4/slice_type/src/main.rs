fn main() {
    let s = String::from("hello world");
    let hello = first_word(&s);
    // we actually can not change s without changing hello, meaning hello would have to be mutable
    // s.clear();
    println!("Hello is {}", hello);
}

fn first_word(s: &str) -> &str {
    // iterate through s and find where the first space is
    let bytes = s.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        // if the byte is a space, return a slice of the string
        if byte == b' ' {
            // this will return a pointer to the memory starting at index 0 until index, not a pointer to s
            return &s[..index];
        }
    }
    // return a pointer to everything s points to
    &s[..]
}
