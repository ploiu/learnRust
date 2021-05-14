enum DifferentTypes {
    Int(i32),
    FLoat(f64),
    Text(String),
}

fn main() {
    // there are a couple ways of creating vectors
    let first = vec![1, 2, 3, 4, 5];
    // must declare as mut to add and remove values
    let mut second = Vec::<i32>::new();
    // this way might be easier though
    let third: Vec<i32> = Vec::new();
    second.push(100);
    println!("{:?}", second);
    // there are multiple ways to access vector elements
    // this will panic if the index is out of bounds
    println!("{}", &first[0]);
    // this way will not panic
    match first.get(100) {
        // take a reference instead of ownership
        Some(&number) => println!("{}", number),
        None => println!("out of bounds"),
    }
    // you can use an enum to group different types for when you need to store them in a Vector
    let different_types = vec![
        DifferentTypes::FLoat(32.0),
        DifferentTypes::Int(6),
        DifferentTypes::Text(String::from("test")),
    ];
    references();
    iteration();
}

fn references() {
    // just like with variables, we can't have a mutable and immutable reference
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // can't reference first here at all because v was changed
    println!("{:?}", v);
}

fn iteration() {
    let mut v = vec![1, 2, 3, 4, 5];
    // rust uses for in syntax, remember to take a references instead of letting the for loop take ownership
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v);
    // we can also modify the vector by getting a mutable reference
    for i in &mut v {
        // dereference i here
        *i += 50;
    }

    println!("{:?}", v);
}
