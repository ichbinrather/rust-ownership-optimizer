fn longest_word(text: &str) -> &str {
    let mut words = text.split(' ');
    let mut best = words.next().unwrap_or("");
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
