use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let different_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // Hash map becomes the owner of values used during inserting

    let team_name = String::from("Blue");
    let score = different_scores.get(&team_name);
}
