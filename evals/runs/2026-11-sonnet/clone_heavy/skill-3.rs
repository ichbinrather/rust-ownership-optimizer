use std::collections::HashMap;

fn find_user(users: &[String], name: &str) -> bool {
    users.iter().any(|u| u == name)
}

fn total_len(words: &[String]) -> usize {
    words.iter().map(String::len).sum()
}

fn main() {
    let users = vec!["ana".to_string(), "bo".to_string()];
    let name = "bo".to_string();
    println!("{}", find_user(&users, &name));
    println!("{}", total_len(&users));

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for u in &users {
        *counts.entry(u.as_str()).or_insert(0) += 1;
    }
    println!("{:?}", counts);
}
