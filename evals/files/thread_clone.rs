// Eval input: compiles, but deep-copies a large Vec into every thread.
use std::thread;

fn main() {
    let data: Vec<u64> = (0..1_000_000).collect();
    let mut handles = Vec::new();
    for i in 0..4 {
        let copy = data.clone();
        handles.push(thread::spawn(move || copy.iter().skip(i).step_by(4).sum::<u64>()));
    }
    let total: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("{total}");
}
