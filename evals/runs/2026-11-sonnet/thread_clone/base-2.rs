use std::sync::Arc;
use std::thread;

fn main() {
    let data: Arc<Vec<u64>> = Arc::new((0..1_000_000).collect());
    let mut handles = Vec::new();
    for i in 0..4 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || data.iter().skip(i).step_by(4).sum::<u64>()));
    }
    let total: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("{total}");
}
