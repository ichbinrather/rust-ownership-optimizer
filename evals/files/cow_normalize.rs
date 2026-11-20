// Eval input: compiles, but normalize allocates a new String for every word.
fn normalize(s: &str) -> String {
    if s.contains(' ') {
        s.replace(' ', "_")
    } else {
        s.to_string()
    }
}

fn main() {
    let words: Vec<String> = (0..200_000)
        .map(|i| if i % 100 == 0 { format!("a b {i}") } else { format!("w{i}") })
        .collect();
    let mut total = 0;
    let mut underscores = 0;
    for w in &words {
        let n = normalize(w);
        total += n.len();
        underscores += n.matches('_').count();
    }
    println!("{total} {underscores}");
}
