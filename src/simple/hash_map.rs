use std::collections::HashMap;

#[allow(dead_code)]
pub fn using_hash_map() {
    let blue_team_name = String::from("Blue");
    let yellow_team_name = String::from("Yellow");

    // Generate HashMap1
    let mut scores = HashMap::new();
    scores.insert(&blue_team_name, 10);
    scores.entry(&yellow_team_name).or_insert(50);
    scores.entry(&blue_team_name).or_insert(30);

    // Generate HashMap2
    let teams = vec![blue_team_name.to_owned(), yellow_team_name.to_owned()];
    let initial_scores = vec![10, 50];
    let scores_mirror: HashMap<_, _> = teams.iter().zip(initial_scores.iter().copied()).collect();

    // Assert
    println!("origin: {:?}", scores);
    println!("mirror: {:?}", scores_mirror);
    assert_score(&scores, &scores_mirror, &blue_team_name);
    assert_score(&scores, &scores_mirror, &yellow_team_name);

    print_score(&scores, &blue_team_name);
    print_score(&scores, &yellow_team_name);
}

fn assert_score(map_a: &HashMap<&String, i32>, map_b: &HashMap<&String, i32>, team_name: &String) {
    assert_eq!(map_a.get(&team_name), map_b.get(&team_name));
}

fn print_score(scores: &HashMap<&String, i32>, team_name: &String) {
    let score = match scores.get(team_name) {
        Some(v) => v,
        None => &0,
    };
    println!("{}'s score: {}", team_name, score);
}
