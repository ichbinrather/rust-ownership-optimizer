// Closures, threads, iterators, conditional borrows, and Cow without needless copies.
// Run: cargo run --example advanced_patterns

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

// ---------------------------------------------------------------------------
// 1. Closures: capture only what you need.
// ---------------------------------------------------------------------------
struct Config {
    prefix: String,
    verbose: bool,
}

// BEFORE: `let cfg = config.clone(); move |x| format!("{}{x}", cfg.prefix)`
// clones the whole struct just to read one field.
// AFTER: borrow the single field; the closure lives no longer than `config`.
fn make_labels(config: &Config, items: &[&str]) -> Vec<String> {
    let prefix = &config.prefix;
    items.iter().map(|item| format!("{prefix}{item}")).collect()
}

// ---------------------------------------------------------------------------
// 2. Threads: scoped threads borrow; spawned threads share with Arc.
// ---------------------------------------------------------------------------

// BEFORE: `let data = data.clone(); thread::spawn(move || ...)` deep-copies the Vec
// for every thread. AFTER: thread::scope lets threads borrow local data, because the
// scope guarantees they finish before `data` is dropped.
fn parallel_sum(data: &[u64]) -> u64 {
    let (left, right) = data.split_at(data.len() / 2);
    thread::scope(|s| {
        let a = s.spawn(|| left.iter().sum::<u64>());
        let b = s.spawn(|| right.iter().sum::<u64>());
        a.join().unwrap() + b.join().unwrap()
    })
}

// When a thread must outlive the caller ('static), share immutable data through Arc.
// Arc::clone only increments a counter; the Vec is never copied.
fn spawn_readers(data: Arc<Vec<u64>>) -> u64 {
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || data.iter().max().copied().unwrap_or(0))
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

// ---------------------------------------------------------------------------
// 3. Iterators: copied() for Copy types, no intermediate collect().
// ---------------------------------------------------------------------------

// BEFORE: `scores.iter().cloned().collect::<Vec<_>>().iter().filter(..).count()`
// AFTER: one lazy pass, no allocation.
fn count_passing(scores: &[u32], threshold: u32) -> usize {
    scores.iter().copied().filter(|&s| s >= threshold).count()
}

// ---------------------------------------------------------------------------
// 4. Conditional borrow return (a known limit of today's borrow checker).
// ---------------------------------------------------------------------------

// BEFORE (rejected by the current borrow checker, E0502/E0499):
//     if let Some(v) = map.get(key) { return v; }
//     map.insert(key.to_string(), default);
//     map.get(key).unwrap()
// The tempting fix is to clone the value out. AFTER: the entry API expresses
// "get or insert" with a single mutable borrow and one lookup.
fn get_or_default<'m>(map: &'m mut HashMap<String, String>, key: &str) -> &'m String {
    map.entry(key.to_string()).or_insert_with(|| String::from("default"))
}

// ---------------------------------------------------------------------------
// 5. Cow: allocate only when the input must change.
// ---------------------------------------------------------------------------

// BEFORE: always returns a new String, even when nothing changes.
#[allow(dead_code)]
fn normalize_before(input: &str) -> String {
    input.replace('\t', "    ")
}

// AFTER: borrows the input in the common case.
fn normalize_after(input: &str) -> Cow<'_, str> {
    if input.contains('\t') {
        Cow::Owned(input.replace('\t', "    "))
    } else {
        Cow::Borrowed(input)
    }
}

// ---------------------------------------------------------------------------
// 6. Graphs: store indices, not references or Rc<RefCell<_>>.
// ---------------------------------------------------------------------------
struct Graph {
    names: Vec<String>,
    edges: Vec<Vec<usize>>, // edges[i] = indices of neighbors of node i
}

impl Graph {
    fn add_node(&mut self, name: &str) -> usize {
        self.names.push(name.to_string());
        self.edges.push(Vec::new());
        self.names.len() - 1
    }

    fn connect(&mut self, a: usize, b: usize) {
        self.edges[a].push(b);
        self.edges[b].push(a);
    }

    fn neighbors(&self, node: usize) -> impl Iterator<Item = &str> + '_ {
        self.edges[node].iter().map(|&i| self.names[i].as_str())
    }
}

fn main() {
    let config = Config { prefix: "item-".into(), verbose: true };
    let labels = make_labels(&config, &["a", "b"]);
    if config.verbose {
        println!("{labels:?}");
    }

    let data: Vec<u64> = (1..=10).collect();
    println!("scoped sum: {}", parallel_sum(&data));
    println!("max x2: {}", spawn_readers(Arc::new(data)));

    println!("passing: {}", count_passing(&[40, 75, 90], 60));

    let mut map = HashMap::new();
    println!("value: {}", get_or_default(&mut map, "theme"));

    for s in ["no tabs", "a\tb"] {
        match normalize_after(s) {
            Cow::Borrowed(b) => println!("borrowed: {b}"),
            Cow::Owned(o) => println!("allocated: {o}"),
        }
    }

    let mut g = Graph { names: Vec::new(), edges: Vec::new() };
    let a = g.add_node("a");
    let b = g.add_node("b");
    g.connect(a, b);
    println!("neighbors of a: {:?}", g.neighbors(a).collect::<Vec<_>>());
}
