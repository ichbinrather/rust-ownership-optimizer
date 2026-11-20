// Borrow conflict fixes without cloning.
// Run: cargo run --example borrow_conflicts

use std::collections::HashMap;

struct Inventory {
    items: Vec<String>,
    log: Vec<String>,
}

impl Inventory {
    // BEFORE (does not compile, E0502):
    //     for item in self.items.iter() {
    //         self.record(item);          // &mut self while self.items is borrowed
    //     }
    // A common "fix" is `for item in self.items.clone()`, which copies every String.
    //
    // AFTER: borrow disjoint fields directly. The borrow checker sees that
    // `self.items` (shared) and `self.log` (mutable) do not overlap.
    fn audit(&mut self) {
        for item in &self.items {
            self.log.push(format!("checked {item}"));
        }
    }
}

// BEFORE: get + insert does two lookups, and the borrow from `get` blocks `insert`.
//     if let Some(c) = counts.get_mut(word) { *c += 1 } else { counts.insert(word.to_string(), 1); }
// AFTER: the entry API does one lookup and needs no clone of the map value.
fn count(words: &[&str]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in words {
        *counts.entry((*word).to_string()).or_insert(0) += 1;
    }
    counts
}

// BEFORE (does not compile, E0502): pushing to a Vec while holding &v[0].
//     let first = &v[0];
//     v.push(*first);
// AFTER: copy the small value out, so the shared borrow ends before the mutation.
fn duplicate_first(v: &mut Vec<i32>) {
    if let Some(&first) = v.first() {
        v.push(first);
    }
}

// Removing elements while iterating: use retain instead of cloning the Vec.
fn drop_empty(v: &mut Vec<String>) {
    v.retain(|s| !s.is_empty());
}

// Taking a field out of &mut self without cloning: std::mem::take.
struct Buffer {
    pending: Vec<u8>,
}

impl Buffer {
    fn flush(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.pending)
    }
}

fn main() {
    let mut inv = Inventory {
        items: vec!["bolt".into(), "nut".into()],
        log: Vec::new(),
    };
    inv.audit();
    println!("{:?}", inv.log);

    println!("{:?}", count(&["a", "b", "a"]));

    let mut v = vec![7, 8];
    duplicate_first(&mut v);
    println!("{v:?}");

    let mut s = vec![String::new(), "x".to_string()];
    drop_empty(&mut s);
    println!("{s:?}");

    let mut buf = Buffer { pending: vec![1, 2, 3] };
    let out = buf.flush();
    println!("flushed {out:?}, left {:?}", buf.pending);
}
