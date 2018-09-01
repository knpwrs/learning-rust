use std::collections::HashMap;

fn main() {
    let mut team_scores = HashMap::new();
    team_scores.insert(String::from("Red"), 10);
    team_scores.insert(String::from("Blue"), 5);

    let teams = vec![String::from("Yellow"), String::from("green")];
    let scores = vec![42, 64];
    let other_team_scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    let field_name = String::from("Favorite Color");
    let field_value = String::from("red");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}: {}", field_name, field_value); // Cannot do this! Ownership was transfered to the map!
    // Values which implement the Copy trait will be copied to the map

    let team = String::from("Red");
    if let Some(score) = team_scores.get(&team) {
        println!("{}: {}", team, score);
    }
    for (key, value) in &team_scores { // use a reference to prevent transferring ownership
        println!("({}, {})", key, value);
    }

    // Check before inserting
    team_scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", team_scores);

    // Update a value based on an old value
    let text = "the quick brown fox jumps over the lazy dog";
    let mut words = HashMap::new();
    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0); // returns a mutable reference to the value
        *count += 1;
    }
    println!("{:?}", words);
}
