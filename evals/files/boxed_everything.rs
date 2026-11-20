// Eval input: compiles, but uses Box where it adds only indirection.
trait Greeter {
    fn greet(&self) -> String;
}

struct English;

impl Greeter for English {
    fn greet(&self) -> String {
        "hello".to_string()
    }
}

fn run(g: Box<dyn Greeter>) -> String {
    g.greet()
}

fn numbers() -> Box<Vec<i32>> {
    Box::new(vec![1, 2, 3])
}

fn main() {
    let count = Box::new(3);
    println!("{} {}", run(Box::new(English)), *count);
    println!("{:?}", numbers());
}
