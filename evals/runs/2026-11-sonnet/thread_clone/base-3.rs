use std::thread;

fn main() {
    let data: Vec<u64> = (0..1_000_000).collect();
    let data = &data[..];
    let total: u64 = thread::scope(|s| {
        let handles: Vec<_> = (0..4)
            .map(|i| s.spawn(move || data.iter().skip(i).step_by(4).sum::<u64>()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });
    println!("{total}");
}
