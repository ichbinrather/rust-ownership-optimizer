fn longest_word(text: &str) -> &str {
    let words: Vec<&str> = text.split(' ').collect();
    let mut best = words[0];
    for w in words {
        if w.len() > best.len() {
            best = w;
        }
    }
    best
}

fn main() {
    println!("{}", longest_word("ownership is checked at compile time"));
}
