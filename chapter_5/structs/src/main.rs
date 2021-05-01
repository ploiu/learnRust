// structs are kind of like tuples but with named elements which are unordered
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// can also create tuple structs, for situations where you want to differentiate between normal tuples without the verbosity of a normal struct
struct Color(u8, u8, u8);

fn main() {
    // we create a struct instance this way
    let mut user = User {
        username: String::from("ploiu"),
        email: String::from("test@test.com"),
        sign_in_count: 0,
        active: true,
    };
    // since the user is mutable, we can alter its field
    user.email = String::from("test2@test.com");
    let user2 = build_user(String::from("test"), String::from("test2"));
    // we can destructure a structure when creating another structure
    let user3 = User {
        username: String::from("test_test"),
        ..user2
    };
    println!("user3's email: {}", user3.email);
    // create a tuple struct this way
    let color = Color(20, 255, 20);
    // use :? to print a debug entry for a struct, but the struct has to implement debug
    println!("the user is {:?}", user);
}

fn build_user(email: String, username: String) -> User {
    // if the field and variable names are the same, we can do it shorthand
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
