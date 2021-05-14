use std::collections::HashMap;

fn main() {
    // like everything, use the mut keyword to be able to add and remove elements
    let blue = String::from("Blue");
    let mut scores = HashMap::new();
    // this moves blue and the map takes ownership of it
    scores.insert(blue, 10);
    scores.insert(String::from("Yellow"), 50);
    // another way is to combine 2 vectors or tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let points = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(points.iter()).collect();
    let team_name = String::from("Blue");
    let blue_score = scores.get(&team_name).unwrap();
    println!("{}", blue_score);
    // we can also iterate over the keys and values in a hash map; order is not guaranteed
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
    let mut scores = HashMap::<String, i32>::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    overwrite_map_value(&mut scores);
    println!("{:?}", scores);
}

fn overwrite_map_value(map: &mut HashMap<String, i32>) {
    // this overwrites any old value for blue that map had
    map.insert(String::from("Blue"), 25);
}

fn insert_if_none(map: &mut HashMap<String, i32>, key: String, value: i32) {
    map.entry(key).or_insert(value);
}