mod pipeline;

use pipeline::Pipeline;

fn main() {
    let pipeline = Pipeline::demo(2_000);
    let totals = pipeline.process_all();
    let sum: u64 = totals.iter().sum();
    println!("{} orders, first {:?}, last {:?}, sum {}", totals.len(), totals.first(), totals.last(), sum);
}
