mod store;

use store::Store;

fn main() {
    let mut store = Store::demo();
    for idx in [0, 1, 0, 2, 1, 0] {
        let s = store.summary(idx);
        println!("{idx}: {s}");
    }
    println!("computed {} summaries", store.computed);
}
