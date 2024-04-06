use std::collections::HashMap;

#[allow(dead_code)]
pub fn using_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Yellow"), String::from("Blue")];
    let initial_scores = vec![10, 50];

    let scores_mirror: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
    println!("{:?}", scores_mirror);
    'root_up: for original in &scores {
        for mirror in &scores_mirror {
            assert_eq!(&original.0, mirror.0);
            assert_eq!(&original.1, mirror.1);
            break 'root_up;
        }
    }
}
