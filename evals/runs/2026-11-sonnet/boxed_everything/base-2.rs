trait Greeter {
    fn greet(&self) -> &'static str;
}

struct English;

impl Greeter for English {
    fn greet(&self) -> &'static str {
        "hello"
    }
}

fn run(g: &dyn Greeter) -> &'static str {
    g.greet()
}

fn numbers() -> Vec<i32> {
    vec![1, 2, 3]
}

fn main() {
    let count = 3;
    println!("{} {}", run(&English), count);
    println!("{:?}", numbers());
}
