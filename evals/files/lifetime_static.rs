// Eval input: does not compile (E0515). Fix without 'static, Box::leak, or clone.
fn longest_word(text: &str) -> &str {
    let words: Vec<String> = text.split(' ').map(|w| w.to_string()).collect();
    let mut best = &words[0];
    for w in &words {
        if w.len() > best.len() {
            best = w;
        }
    }
    best
}

fn main() {
    println!("{}", longest_word("ownership is checked at compile time"));
}
