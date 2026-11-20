// Eval input: compiles, but clones far more than needed. Remove needless copies.
use std::collections::HashMap;

fn find_user(users: Vec<String>, name: String) -> bool {
    users.contains(&name)
}

fn total_len(words: &Vec<String>) -> usize {
    let mut total = 0;
    for w in words.clone() {
        total += w.clone().len();
    }
    total
}

fn main() {
    let users = vec!["ana".to_string(), "bo".to_string()];
    let name = "bo".to_string();
    println!("{}", find_user(users.clone(), name.clone()));
    println!("{}", total_len(&users));

    let mut counts: HashMap<String, u32> = HashMap::new();
    for u in users.iter() {
        let key = u.clone();
        if counts.contains_key(&key) {
            *counts.get_mut(&key).unwrap() += 1;
        } else {
            counts.insert(key.clone(), 1);
        }
    }
    println!("{:?}", counts);
}
