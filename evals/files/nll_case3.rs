// Eval input: does not compile (conditional borrow return). Fix it without cloning the value.
use std::collections::HashMap;

struct Cache {
    map: HashMap<u32, String>,
}

impl Cache {
    fn get_or_compute(&mut self, key: u32) -> &String {
        if let Some(v) = self.map.get(&key) {
            return v;
        }
        let value = format!("value-{}", key * 2);
        self.map.insert(key, value);
        self.map.get(&key).unwrap()
    }
}

fn main() {
    let mut cache = Cache { map: HashMap::new() };
    let a = cache.get_or_compute(1).len();
    let b = cache.get_or_compute(1).len();
    let c = cache.get_or_compute(21).len();
    println!("{a} {b} {c} {}", cache.map.len());
}
