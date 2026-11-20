// Unnecessary clone() examples: before and after.
// Run: cargo run --example unnecessary_clone

// BEFORE: takes ownership of a String it only reads, forcing callers to clone.
#[allow(dead_code)]
fn count_words_before(text: String) -> usize {
    text.split_whitespace().count()
}

// AFTER: borrows a &str. Works with String, &String, and string literals; no copy.
fn count_words_after(text: &str) -> usize {
    text.split_whitespace().count()
}

// BEFORE: clones every element to search the list.
#[allow(dead_code, clippy::ptr_arg)]
fn contains_before(names: &[String], target: &String) -> bool {
    names.iter().cloned().any(|n| n == *target)
}

// AFTER: compares borrowed &str values; no allocation.
fn contains_after(names: &[String], target: &str) -> bool {
    names.iter().any(|n| n == target)
}

// BEFORE: clones an Option<String> just to inspect it.
#[allow(dead_code)]
fn greeting_before(name: &Option<String>) -> usize {
    match name.clone() {
        Some(n) => n.len(),
        None => 0,
    }
}

// AFTER: as_deref() turns &Option<String> into Option<&str>.
fn greeting_after(name: &Option<String>) -> usize {
    name.as_deref().map_or(0, str::len)
}

fn main() {
    let text = String::from("ownership is checked at compile time");

    // BEFORE: let n = count_words_before(text.clone()); // clone only to keep `text`
    let n = count_words_after(&text);
    println!("{n} words in: {text}");

    let names = vec!["alice".to_string(), "bob".to_string()];
    println!("contains bob: {}", contains_after(&names, "bob"));

    let name = Some("carol".to_string());
    println!("name length: {}", greeting_after(&name));

    // Last use: move instead of clone. `text` is dropped when `owned` goes out of scope.
    let owned = text;
    println!("moved: {owned}");
}
