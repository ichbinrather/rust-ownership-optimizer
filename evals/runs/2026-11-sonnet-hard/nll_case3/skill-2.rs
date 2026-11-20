use std::collections::HashMap;

struct Cache {
    map: HashMap<u32, String>,
}

impl Cache {
    fn get_or_compute(&mut self, key: u32) -> &String {
        self.map
            .entry(key)
            .or_insert_with(|| format!("value-{}", key * 2))
    }
}

fn main() {
    let mut cache = Cache { map: HashMap::new() };
    let a = cache.get_or_compute(1).len();
    let b = cache.get_or_compute(1).len();
    let c = cache.get_or_compute(21).len();
    println!("{a} {b} {c} {}", cache.map.len());
}
