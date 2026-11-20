// Unnecessary Box examples: before and after.
// Run: cargo run --example unnecessary_box

trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

// BEFORE: Box<Vec<T>> adds a second heap indirection; Vec already owns heap storage.
#[allow(dead_code, clippy::box_collection, clippy::boxed_local)]
fn sum_before(values: Box<Vec<i32>>) -> i32 {
    values.iter().sum()
}

// AFTER: borrow a slice.
fn sum_after(values: &[i32]) -> i32 {
    values.iter().sum()
}

// BEFORE: dynamic dispatch through a Box although the concrete type is known.
#[allow(dead_code, clippy::boxed_local)]
fn total_area_before(shape: Box<dyn Shape>) -> f64 {
    shape.area()
}

// AFTER: generic parameter; static dispatch, no heap allocation, can be inlined.
fn total_area_after(shape: &impl Shape) -> f64 {
    shape.area()
}

// BEFORE: Box<dyn Iterator> allocates and blocks inlining.
#[allow(dead_code)]
fn evens_before(limit: u32) -> Box<dyn Iterator<Item = u32>> {
    Box::new((0..limit).filter(|n| n % 2 == 0))
}

// AFTER: impl Iterator returns the concrete type with zero allocation.
fn evens_after(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit).filter(|n| n % 2 == 0)
}

// JUSTIFIED: a recursive type needs Box to have a known size at compile time.
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let values = vec![1, 2, 3];
    println!("sum: {}", sum_after(&values));

    let square = Square(3.0);
    println!("area: {}", total_area_after(&square));

    let evens: Vec<u32> = evens_after(10).collect();
    println!("evens: {evens:?}");

    let _list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}
