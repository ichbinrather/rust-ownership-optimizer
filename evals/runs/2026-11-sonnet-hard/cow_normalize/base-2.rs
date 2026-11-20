fn main() {
    let words: Vec<String> = (0..200_000)
        .map(|i| if i % 100 == 0 { format!("a b {i}") } else { format!("w{i}") })
        .collect();
    let mut total = 0;
    let mut underscores = 0;
    for w in &words {
        // Normalizing replaces ' ' with '_' (same byte length), so count directly.
        total += w.len();
        underscores += w.bytes().filter(|&b| b == b' ' || b == b'_').count();
    }
    println!("{total} {underscores}");
}
